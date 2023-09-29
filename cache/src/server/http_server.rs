use crate::lru::disk_lru::{self, DiskLRU};
use crate::server::http_server;

extern crate protos;

//use protos::configuration;

use actix_files::NamedFile;
use actix_multipart::form::tempfile::TempFile;
use actix_multipart::form::MultipartForm;
use actix_multipart::form::MultipartFormConfig;
use actix_web::error::Error;
use actix_web::{middleware::Logger, web, App, HttpServer};
use actix_web::{HttpResponse, Responder};

pub struct Server {
    disk_lru: DiskLRU,
}

impl Clone for Server {
    fn clone(&self) -> Self {
        Server {
            disk_lru: self.disk_lru.clone(),
        }
    }
}

#[derive(Debug, MultipartForm)]
pub struct UploadForm {
    #[multipart(rename = "file")]
    file: TempFile,
}

impl Server {
    pub fn new(d: DiskLRU) -> Server {
        Server { disk_lru: d }
    }

    pub async fn download(&self, path: String) -> Result<NamedFile, Error> {
        self.disk_lru.get_file(path).await
    }

    pub async fn upload(
        &self,
        path: String,
        MultipartForm(form): MultipartForm<UploadForm>,
    ) -> Result<impl Responder, Error> {
        let f = form.file;
        f.file.persist(&path).unwrap();
        log::info!("persist file {} successful", path);

        self.disk_lru.add_file(path).await?;

        Ok(HttpResponse::Ok())
    }
}

pub async fn start_server_and_wait(
    disk_lru: disk_lru::DiskLRU,
    server_config: &protos::configuration::HttpServerConfig,
) -> std::io::Result<()> {
    let server = http_server::Server::new(disk_lru);

    log::info!(
        "starting HTTP server at http://{}:{}",
        server_config.ip_address,
        server_config.port
    );

    HttpServer::new(move || {
        App::new()
            .wrap(Logger::default())
            .app_data(web::Data::new(server.clone()))
            .app_data(MultipartFormConfig::default().total_limit(5000 * 1024 * 1024))
            .service(
                web::resource("/download/{path:.*}")
                    .route(web::get().to(http_server::handlers::download)),
            )
            .service(
                web::resource("/upload/{path:.*}")
                    .route(web::post().to(http_server::handlers::upload)),
            )
    })
    .bind((server_config.ip_address.clone(), server_config.port as u16))?
    .run()
    .await
}

pub mod handlers {
    use crate::server::http_server::Server;
    use crate::server::http_server::UploadForm;
    use actix_files::NamedFile;
    use actix_multipart::form::MultipartForm;
    use actix_web::error::Error;
    use actix_web::{web, Responder};

    pub async fn download(
        path: web::Path<String>,
        server: web::Data<Server>,
    ) -> actix_web::Result<NamedFile> {
        let path = format!("{path}");
        server.download(path).await
    }

    pub async fn upload(
        path: web::Path<String>,
        server: web::Data<Server>,
        form: MultipartForm<UploadForm>,
    ) -> Result<impl Responder, Error> {
        let path = format!("{path}");
        server.upload(path, form).await
    }
}
