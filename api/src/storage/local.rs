// In api/src/storage/local.rs
use async_trait::async_trait;
use std::path::{Path, PathBuf};
use tempfile::NamedTempFile;
use anyhow::Result;
use tokio::fs;
use std::io::Write;
use std::fs as std_fs;

pub struct LocalFileStorage {
    base_path: PathBuf,
}

impl LocalFileStorage {
    pub fn new(base_path: impl Into<PathBuf>) -> Self {
        let base_path = base_path.into();
        // Ensure the directory exists
        std::fs::create_dir_all(&base_path).unwrap_or_else(|_| {
            println!("Failed to create directory: {:?}", base_path);
        });
        Self { base_path }
    }
}

impl Clone for LocalFileStorage {
    fn clone(&self) -> Self {
        Self {
            base_path: self.base_path.clone(),
        }
    }
}

#[async_trait]
impl super::gcs::FileStorage for LocalFileStorage {
    async fn upload_file(&self, file_path: &Path, _bucket: &str, object_name: &str) -> Result<String> {
        let destination = self.base_path.join(object_name);
        
        // Create parent directories if they don't exist
        if let Some(parent) = destination.parent() {
            std_fs::create_dir_all(parent)?;
        }
        
        // Copy the file
        fs::copy(file_path, &destination).await?;
        
        Ok(format!("local://{}", destination.display()))
    }

    async fn download_file(&self, _bucket: &str, object_name: &str) -> Result<NamedTempFile> {
        let source = self.base_path.join(object_name);
        let temp_file = NamedTempFile::new()?;
        let bytes = fs::read(&source).await?;
        std_fs::write(temp_file.path(), bytes)?;
        Ok(temp_file)
    }
}