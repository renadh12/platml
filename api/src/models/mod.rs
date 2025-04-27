use serde::{Deserialize, Serialize};
use chrono::Utc;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Model {
    pub id: String,
    pub name: String,
    pub version: String,
    pub status: ModelStatus,
    pub created_at: chrono::DateTime<Utc>,
    pub updated_at: chrono::DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ModelStatus {
    Created,
    Uploading,
    Active,
    Inactive,
    Error(String),
}

#[derive(Debug, Deserialize)]
pub struct CreateModelRequest {
    pub name: String,
    pub version: String,
}

#[derive(Debug, Serialize)]
pub struct ModelResponse {
    pub id: String,
    pub name: String,
    pub version: String,
    pub status: String,
    pub created_at: String,
    pub updated_at: String,
}

impl From<Model> for ModelResponse {
    fn from(model: Model) -> Self {
        Self {
            id: model.id,
            name: model.name,
            version: model.version,
            status: model.status.to_string(),
            created_at: model.created_at.to_rfc3339(),
            updated_at: model.updated_at.to_rfc3339(),
        }
    }
}

impl ModelStatus {
    pub fn to_string(&self) -> String {
        match self {
            ModelStatus::Created => "CREATED".to_string(),
            ModelStatus::Uploading => "UPLOADING".to_string(),
            ModelStatus::Active => "ACTIVE".to_string(),
            ModelStatus::Inactive => "INACTIVE".to_string(),
            ModelStatus::Error(msg) => format!("ERROR: {}", msg),
        }
    }
} 