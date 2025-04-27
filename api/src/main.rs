mod models;
mod storage;

use axum::{
    routing::{get, post, delete},
    http::StatusCode,
    Json, Router,
    extract::{Path, Multipart},
    response::IntoResponse,
};
use std::net::SocketAddr;
use uuid::Uuid;
use tower_http::cors::CorsLayer;
use chrono::Utc;
use crate::models::{Model, CreateModelRequest, ModelResponse, ModelStatus};
use crate::storage::ModelStorage;
use std::io::Write;

#[derive(Debug)]
struct AppError(anyhow::Error);

impl IntoResponse for AppError {
    fn into_response(self) -> axum::response::Response {
        (
            StatusCode::INTERNAL_SERVER_ERROR,
            format!("Something went wrong: {}", self.0),
        ).into_response()
    }
}

impl<E> From<E> for AppError
where
    E: Into<anyhow::Error>,
{
    fn from(err: E) -> Self {
        Self(err.into())
    }
}

#[tokio::main]
async fn main() {
    // Initialize tracing
    tracing_subscriber::fmt::init();

    // Initialize storage
    let storage = ModelStorage::new();

    // Build our application with routes
    let app = Router::new()
        .route("/", get(health_check))
        .route("/models", post(create_model))
        .route("/models", get(list_models))
        .route("/models/:id", get(get_model))
        .route("/models/:id", delete(delete_model))
        .route("/models/:id/upload", post(upload_model_file))
        .with_state(storage)
        .layer(CorsLayer::permissive());

    // Run the server
    let addr = SocketAddr::from(([0, 0, 0, 0], 8081));
    println!("API service listening on {}", addr);
    axum::serve(tokio::net::TcpListener::bind(addr).await.unwrap(), app)
        .await
        .unwrap();
}

// Health check endpoint
async fn health_check() -> &'static str {
    "Model Management API is healthy"
}

// Create a new model
async fn create_model(
    state: axum::extract::State<ModelStorage>,
    Json(payload): Json<CreateModelRequest>,
) -> Result<(StatusCode, Json<ModelResponse>), AppError> {
    let model = Model {
        id: Uuid::new_v4().to_string(),
        name: payload.name,
        version: payload.version,
        status: ModelStatus::Created,
        created_at: Utc::now(),
        updated_at: Utc::now(),
    };

    let model = state.create_model(model).await.map_err(|e| anyhow::anyhow!(e))?;
    Ok((StatusCode::CREATED, Json(model.into())))
}

// List all models
async fn list_models(
    state: axum::extract::State<ModelStorage>,
) -> Result<Json<Vec<ModelResponse>>, AppError> {
    let models = state.list_models().await;
    Ok(Json(models.into_iter().map(|m| m.into()).collect()))
}

// Get a model by ID
async fn get_model(
    state: axum::extract::State<ModelStorage>,
    Path(id): Path<String>,
) -> Result<Json<ModelResponse>, AppError> {
    let model = state.get_model(&id).await
        .ok_or_else(|| anyhow::anyhow!("Model not found"))?;
    Ok(Json(model.into()))
}

// Delete a model
async fn delete_model(
    state: axum::extract::State<ModelStorage>,
    Path(id): Path<String>,
) -> Result<StatusCode, AppError> {
    if state.delete_model(&id).await {
        Ok(StatusCode::NO_CONTENT)
    } else {
        Err(anyhow::anyhow!("Model not found").into())
    }
}

// Upload model file
async fn upload_model_file(
    state: axum::extract::State<ModelStorage>,
    Path(model_id): Path<String>,
    mut multipart: Multipart,
) -> Result<Json<serde_json::Value>, AppError> {
    // Iterate through the received multipart fields.
    while let Some(field) = multipart.next_field().await? {
        // Debug output: Print the field name
        println!("Received multipart field: {:?}", field.name());
        // Alternatively, if you're using tracing:
        // tracing::debug!("Received multipart field: {:?}", field.name());

        if field.name() == Some("file") {
            let data = field.bytes().await?;
            // Create a temporary file and write the file bytes
            let temp_file = tempfile::NamedTempFile::new()?;
            std::fs::write(temp_file.path(), data)?;
            
            // Upload the model file to GCS and update the model's status.
            let gcs_path = state.upload_model_file(&model_id, temp_file.path()).await?;
            return Ok(Json(serde_json::json!({
                "status": "success",
                "gcs_path": gcs_path
            })));
        }
    }
    
    Err(anyhow::anyhow!("No file uploaded").into())
}