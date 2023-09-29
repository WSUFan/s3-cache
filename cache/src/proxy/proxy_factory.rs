use crate::proxy::proxy::StorageProxy;
use crate::proxy::s3_proxy;
use async_trait::async_trait;
use protos::configuration::application_configuration::ProxyConfig::S3ProxyConfig;
use protos::configuration::ApplicationConfiguration;

pub struct EmptyProxy {}

#[async_trait]
impl StorageProxy for EmptyProxy {
    async fn put_object(&self) {}
    fn get_object(&self) {}
    fn check_object(&self) {}
}

pub async fn create_proxy(app_config: &ApplicationConfiguration) -> Box<dyn StorageProxy> {
    log::debug!("{:?}", app_config);
    match app_config.ProxyConfig.as_ref().unwrap() {
        S3ProxyConfig(proxy) => Box::new(s3_proxy::S3Proxy::new_from_config(proxy).await.unwrap()),
        _ => Box::new(EmptyProxy {}),
    }
}
