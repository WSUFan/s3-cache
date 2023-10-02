use crate::proxy::proxy::StorageProxy;
use actix_web::body::MessageBody;
use async_trait::async_trait;
use awscreds::Credentials;
use protos::configuration::S3ProxyConfig;
use s3::error::S3Error;
use s3::Bucket;
use std::path::Path;

pub struct S3Proxy {
    bucket: Bucket,
}

#[async_trait]
impl StorageProxy for S3Proxy {
    async fn put_object(&self, path: &Path) {
        match std::fs::read(path) {
            Err(e) => log::warn!("read file error: {}", e),
            Ok(c) => match self
                .bucket
                .put_object("./", &c.try_into_bytes().expect("converting bytes failed"))
                .await
            {
                Ok(v) => {
                    log::info!("put object {} to the bucket successfull", v)
                }
                Err(err) => {
                    log::warn!("put object to the bucket failure: {}", err)
                }
            },
        }
    }
    fn get_object(&self) {}
    fn check_object(&self) {}
}

impl S3Proxy {
    pub async fn new_from_config(proxy: &S3ProxyConfig) -> Result<S3Proxy, S3Error> {
        log::info!("use s3 proxy");
        let credentials = Credentials::new(
            Some(&proxy.access_key),
            Some(&proxy.secret_id),
            None,
            None,
            None,
        )
        .expect("create aws credentials failed");

        let bucket = Bucket::new(&proxy.bucket, proxy.region.parse()?, credentials)?;

        Ok(S3Proxy { bucket: bucket })
    }
}
