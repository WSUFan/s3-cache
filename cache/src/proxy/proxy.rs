use async_trait::async_trait;
use std::path::Path;

#[async_trait]
pub trait StorageProxy: Send + Sync {
    async fn put_object(&self, path: &Path);
    fn get_object(&self);
    fn check_object(&self);
}
