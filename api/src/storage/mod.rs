use std::sync::Arc;
use tokio::sync::RwLock;
use std::collections::HashMap;
use crate::models::Model;
use crate::storage::gcs::FileStorage; // Removed GCSStorage
use anyhow::Result;

mod gcs;
mod local;

// Remove the Debug derive
#[derive(Clone)]
pub struct ModelStorage {
    models: Arc<RwLock<HashMap<String, Model>>>,
    file_storage: Arc<dyn FileStorage + Send + Sync>, // Added Send + Sync bounds
}

// Manually implement Debug
impl std::fmt::Debug for ModelStorage {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("ModelStorage")
            .field("models", &self.models)
            .field("file_storage", &"<dyn FileStorage>")
            .finish()
    }
}

impl ModelStorage {
    pub fn new() -> Self {
        // Use local storage for testing
        let file_storage = Arc::new(local::LocalFileStorage::new("./model_storage"));
        
        Self {
            models: Arc::new(RwLock::new(HashMap::new())),
            file_storage,
        }
    }

    pub async fn create_model(&self, model: Model) -> Result<Model> {
        let mut models = self.models.write().await;
        if models.contains_key(&model.id) {
            anyhow::bail!("Model with this ID already exists");
        }
        models.insert(model.id.clone(), model.clone());
        Ok(model)
    }

    pub async fn get_model(&self, id: &str) -> Option<Model> {
        let models = self.models.read().await;
        models.get(id).cloned()
    }

    pub async fn delete_model(&self, id: &str) -> bool {
        let mut models = self.models.write().await;
        models.remove(id).is_some()
    }

    pub async fn list_models(&self) -> Vec<Model> {
        let models = self.models.read().await;
        models.values().cloned().collect()
    }

    pub async fn upload_model_file(&self, model_id: &str, file_path: &std::path::Path) -> Result<String> {
        let model = self.get_model(model_id)
            .await
            .ok_or_else(|| anyhow::anyhow!("Model not found"))?;

        let object_name = format!("models/{}/{}.model", model_id, model.version);
        let gcs_path = self.file_storage.upload_file(file_path, "ml-platform-models", &object_name).await?;

        // Update model status
        let mut models = self.models.write().await;
        if let Some(model) = models.get_mut(model_id) {
            model.status = crate::models::ModelStatus::Active;
        }

        Ok(gcs_path)
    }
}