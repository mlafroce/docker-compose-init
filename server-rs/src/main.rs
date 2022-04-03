use crate::server::Server;
use envconfig::Envconfig;
use log::info;

use configparser::ini::Ini;
mod client;
mod server;

/// Configuration loaded from environment
#[derive(Envconfig)]
struct Config {
    #[envconfig(from = "LOGGING_LEVEL")]
    logging_level: Option<String>,
    #[envconfig(from = "SERVER_PORT")]
    server_port: Option<String>,
    // Value hardcoded in std::net::TcpListener, cannot change using standard sockets
    // server_listen_backlog: u32,
}

fn main() {
    let env_config = Config::init().unwrap();

    let mut ini_config = Ini::new();
    let map = ini_config.load("config.ini").unwrap_or_default();
    let ini_port = ini_config.get("DEFAULT", "server_port");

    let logging_level = ini_config
        .get("DEFAULT", "logging_level")
        .or(env_config.logging_level)
        .unwrap();
    println!("Setting logger level: {}", logging_level);
    std::env::set_var("RUST_LOG", logging_level);

    env_logger::init();
    info!("Config loaded");

    let port = ini_config
        .get("DEFAULT", "server_port")
        .or(env_config.server_port)
        .unwrap();
    let addr = format!("0.0.0.0:{}", port);

    info!("Server starting on port {}", port);
    let mut server = Server::new(&addr).unwrap();
    server.run().unwrap();
    info!("Server exit");
}
