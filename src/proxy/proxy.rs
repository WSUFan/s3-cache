use async_trait::async_trait;

#[async_trait]
pub trait StorageProxy: Send + Sync {
    async fn put_object(&self);
    fn get_object(&self);
    fn check_object(&self);
}
