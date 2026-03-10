mod errors;
mod config;
mod services;
mod api;
use clap::Parser;
use pingora::server::Server;
use tracing::info;
use errors::ProxyResult;

fn main() -> ProxyResult<()> {
    let config = config::load()?;
    tracing_subscriber::fmt()
        .with_env_filter(&config.logging.as_ref().map(|l| l.level.clone()).unwrap_or_else(|| "info".into()))
        .init();
    info!("Starting locci-proxy in {:?} mode", config.mode);
    let mut server = Server::new(Some(config.server.bind_address.parse()?))?;
    server.bootstrap();
    let proxy_service = services::build_service(&config)?;
    server.add_service(proxy_service);
    if config.control_api.enabled {
        let api_addr = config.control_api.bind_address.parse()?;
        let api_key = config.control_api.api_key.clone();
        let config_clone = config.clone();
        tokio::spawn(async move { api::run(api_addr, api_key, config_clone).await });
    }
    info!("Proxy listening on {}", config.server.bind_address);
    server.run_forever();
}
