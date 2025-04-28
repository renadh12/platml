use axum::{
    routing::{get, post, delete},
    http::StatusCode,
    Json, Router,
    extract::{Path, State},
};
use serde::{Deserialize, Serialize};
use std::net::SocketAddr;
use std::path::PathBuf;
use std::sync::Arc;
use tokio::sync::RwLock;
use tower_http::cors::CorsLayer;

#[derive(Debug, Clone)]
struct ModelState {
    model_path: Option<String>,
    model_data: Option<Vec<u8>>,
}

#[derive(Debug, Clone)]
struct AppState {
    model: Arc<RwLock<ModelState>>,
    models_dir: PathBuf, // Path to models directory
}

#[tokio::main]
async fn main() {
    // Initialize tracing
    tracing_subscriber::fmt::init();

    // Initialize app state
    let state = AppState {
        model: Arc::new(RwLock::new(ModelState {
            model_path: None,
            model_data: None,
        })),
        models_dir: PathBuf::from("/app/model_storage/models"), // Match the path used in model management API
    };

    // Ensure the models directory exists
    std::fs::create_dir_all(&state.models_dir).unwrap_or_else(|_| {
        println!("Failed to create models directory: {:?}", state.models_dir);
    });

    // Build our application with routes
    let app = Router::new()
        .route("/", get(health_check))
        .route("/predict", post(predict))
        .route("/models/:model_id", post(load_model))
        .route("/models/:model_id", delete(delete_model))
        .route("/models", get(list_models))
        .with_state(state)
        .layer(CorsLayer::permissive());

    // Run the server
    let addr = SocketAddr::from(([0, 0, 0, 0], 8080));
    println!("Serving app listening on {}", addr);
    axum::serve(tokio::net::TcpListener::bind(addr).await.unwrap(), app)
        .await
        .unwrap();
}

// Health check endpoint
async fn health_check() -> &'static str {
    "ML Serving API is healthy"
}

// Request models
#[derive(Deserialize)]
struct PredictionRequest {
    features: Vec<f32>,
}

// Response models
#[derive(Serialize)]
struct PredictionResponse {
    prediction: f32,
    confidence: f32,
}

async fn load_model(
    state: State<AppState>,
    Path(model_id): Path<String>,
) -> Result<String, (StatusCode, String)> {
    println!("Loading model with ID: {}", model_id);
    println!("Looking for model in directory: {:?}", state.models_dir.join(&model_id));
    
    // Look for model files in the models directory
    let model_dir = state.models_dir.join(&model_id);
    
    if !model_dir.exists() {
        println!("Directory does not exist: {:?}", model_dir);
        return Err((StatusCode::NOT_FOUND, format!("Model {} not found", model_id)));
    }
    
    // Find the model file (assumes there's only one .model file per directory)
    let model_files = match std::fs::read_dir(&model_dir) {
        Ok(dir) => dir
            .filter_map(Result::ok)
            .filter(|entry| {
                entry.path().extension().map_or(false, |ext| ext == "model")
            })
            .collect::<Vec<_>>(),
        Err(e) => {
            println!("Failed to read directory: {:?}", e);
            return Err((StatusCode::INTERNAL_SERVER_ERROR, 
                        format!("Failed to read model directory: {}", e)));
        }
    };
    
    if model_files.is_empty() {
        println!("No .model files found in directory");
        // Try to list all files in the directory for debugging
        if let Ok(all_files) = std::fs::read_dir(&model_dir) {
            println!("Files in directory:");
            for file in all_files.filter_map(Result::ok) {
                println!("  {:?}", file.path());
            }
        }
        return Err((StatusCode::NOT_FOUND, format!("No model file found for {}", model_id)));
    }
    
    let model_path = model_files[0].path();
    println!("Found model file: {:?}", model_path);
    
    let model_data = match std::fs::read(&model_path) {
        Ok(data) => data,
        Err(e) => {
            println!("Failed to read model file: {:?}", e);
            return Err((StatusCode::INTERNAL_SERVER_ERROR, 
                        format!("Failed to read model file: {}", e)));
        }
    };
    
    let mut model_state = state.model.write().await;
    model_state.model_path = Some(model_path.to_string_lossy().to_string());
    model_state.model_data = Some(model_data);
    
    println!("Model loaded successfully");
    Ok("Model loaded successfully".to_string())
}

async fn delete_model(
    state: State<AppState>,
    Path(model_id): Path<String>,
) -> Result<String, (StatusCode, String)> {
    let mut model_state = state.model.write().await;
    
    if model_state.model_path.is_none() {
        return Err((StatusCode::NOT_FOUND, "No model is currently loaded".to_string()));
    }
    
    // Check if the loaded model is the one being requested for deletion
    let current_model_path = model_state.model_path.as_ref().unwrap();
    if !current_model_path.contains(&model_id) {
        return Err((StatusCode::NOT_FOUND, 
            format!("Model {} is not loaded. Currently loaded model has a different ID.", model_id)
        ));
    }
    
    // Clear the model data
    model_state.model_path = None;
    model_state.model_data = None;
    
    Ok(format!("Model {} deleted successfully", model_id))
}

async fn list_models(
    state: State<AppState>,
) -> Result<Json<serde_json::Value>, (StatusCode, String)> {
    use serde_json::json;
    
    let model_state = state.model.read().await;
    
    let model_info = if let Some(path) = &model_state.model_path {
        // Extract the model ID from the path
        let parts: Vec<&str> = path.split('/').collect();
        let model_id = parts.iter()
            .find(|s| s.contains("model"))
            .unwrap_or(&"unknown")
            .to_string();
        
        json!({
            "id": model_id,
            "path": path,
            "size_bytes": model_state.model_data.as_ref().map(|data| data.len()).unwrap_or(0)
        })
    } else {
        json!({ "loaded": false })
    };
    
    Ok(Json(json!({ "model": model_info })))
}

async fn predict(
    state: State<AppState>,
    Json(payload): Json<PredictionRequest>,
) -> Result<Json<PredictionResponse>, (StatusCode, String)> {
    let model_state = state.model.read().await;
    
    if model_state.model_data.is_none() {
        return Err((StatusCode::BAD_REQUEST, "No model loaded".to_string()));
    }

    // Simple rule-based Iris classifier
    // [sepal length, sepal width, petal length, petal width]
    let features = &payload.features;
    
    // Make sure we have exactly 4 features for Iris
    if features.len() != 4 {
        return Err((StatusCode::BAD_REQUEST, 
            format!("Expected 4 features for Iris dataset, got {}", features.len())
        ));
    }
    
    // Simple rule-based classification:
    // - If petal length (features[2]) < 2.5 cm → Setosa (class 0)
    // - If petal length < 5.0 cm → Versicolor (class 1)
    // - Otherwise → Virginica (class 2)
    let petal_length = features[2];
    
    let (prediction, confidence) = if petal_length < 2.5 {
        (0.0, 0.95) // Setosa with 95% confidence
    } else if petal_length < 5.0 {
        (1.0, 0.85) // Versicolor with 85% confidence
    } else {
        (2.0, 0.8) // Virginica with 80% confidence
    };

    let response = PredictionResponse {
        prediction,
        confidence,
    };

    Ok(Json(response))
}