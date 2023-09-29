use protos::configuration::application_configuration::ServerConfig::HttpServerConfig;
use protos::configuration::ApplicationConfiguration;

use cache::cmd::cmd::CommandLineOption;
use cache::load;
use cache::server::http_server;
use protobuf_json_mapping::parse_from_str;

#[actix_web::main] // or #[tokio::main]
async fn main() -> std::io::Result<()> {
    env_logger::init_from_env(env_logger::Env::new().default_filter_or("info"));
    let cli = CommandLineOption::new();
    let json_str = std::fs::read_to_string(cli.config).expect("read file failure");
    let mut app_config = match parse_from_str::<ApplicationConfiguration>(&json_str) {
        Ok(v) => v,
        Err(e) => {
            panic!("parsing JSON fails: {}", e);
        }
    };
    app_config.max_size *= 1024 * 1024 * 1024;
    log::debug!("{}", app_config);

    log::info!(
        "scanning dir {} to build the index",
        app_config.data_directory_path
    );

    let scan_results = load::load_from_disk::scan_dir(&app_config.data_directory_path);
    let disk_lru = load::load_from_disk::create_and_add_to_lru(scan_results, &app_config).await;
    log::info!("finish building the index");

    match app_config
        .ServerConfig
        .expect("server type must be configured")
    {
        HttpServerConfig(server_config) => {
            http_server::start_server_and_wait(disk_lru, &server_config).await
        }
        _ => {
            panic!("unknown server type!")
        }
    }
}
