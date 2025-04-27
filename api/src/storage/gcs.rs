use async_trait::async_trait;
use cloud_storage::Client;
use std::path::Path;
use tempfile::NamedTempFile;
use anyhow::Result;
use std::io::Write;

#[async_trait]
pub trait FileStorage: Send + Sync {
    async fn upload_file(&self, file_path: &Path, bucket: &str, object_name: &str) -> Result<String>;
    async fn download_file(&self, bucket: &str, object_name: &str) -> Result<NamedTempFile>;
}

#[derive(Debug)]
pub struct GCSStorage {
    client: Client,
}

impl GCSStorage {
    pub fn new() -> Self {
        Self {
            client: Client::default(),
        }
    }
}

impl Clone for GCSStorage {
    fn clone(&self) -> Self {
        Self {
            client: Client::default(),
        }
    }
}

#[async_trait]
impl FileStorage for GCSStorage {
    async fn upload_file(&self, file_path: &Path, bucket: &str, object_name: &str) -> Result<String> {
        let bytes = tokio::fs::read(file_path).await?;
        self.client.object().create(bucket, bytes, object_name, "application/octet-stream").await?;
        Ok(format!("gs://{}/{}", bucket, object_name))
    }

    async fn download_file(&self, bucket: &str, object_name: &str) -> Result<NamedTempFile> {
        let bytes = self.client.object().download(bucket, object_name).await?;
        let temp_file = NamedTempFile::new()?;
        std::fs::write(temp_file.path(), bytes)?;
        Ok(temp_file)
    }
} 