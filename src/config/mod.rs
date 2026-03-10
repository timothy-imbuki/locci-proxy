use std::collections::HashMap;
use std::fs;
use serde::{Deserialize, Serialize};
use dotenvy::dotenv;
use regex::Regex;
use crate::errors::{ProxyError, ProxyResult};
pub mod cli;
pub mod api;
use cli::CliArgs;
pub use api::ControlApiConfig;

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(tag = "mode", rename_all = "snake_case")]
pub enum OperationMode { LoadBalancer, ApiGateway }

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct ServerConfig { pub bind_address: String, pub workers: Option<usize> }

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct LoggingConfig { pub level: String }

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct HealthCheckConfig { pub interval_secs: u64, pub timeout_secs: u64, pub path: Option<String> }

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct UpstreamConfig {
    pub servers: Vec<String>,
    pub strategy: String,
    pub tls: Option<bool>,
    pub sni: Option<String>,
    pub health_check: Option<HealthCheckConfig>,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct LoadBalancerConfig { pub upstream: String }

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct RouteConfig {
    pub path_pattern: String,
    pub methods: Vec<String>,
    pub upstream: String,
    pub strip_prefix: Option<bool>,
    pub timeout_secs: Option<u64>,
    pub middlewares: Vec<String>,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct MiddlewareConfig {}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct ApiGatewayConfig {
    pub routes: HashMap<String, RouteConfig>,
    pub middlewares: MiddlewareConfig,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct UnifiedConfig {
    pub mode: OperationMode,
    pub server: ServerConfig,
    pub logging: Option<LoggingConfig>,
    pub upstreams: HashMap<String, UpstreamConfig>,
    pub load_balancer: Option<LoadBalancerConfig>,
    pub api_gateway: Option<ApiGatewayConfig>,
    pub control_api: ControlApiConfig,
}

#[derive(Clone)]
pub struct CompiledRoute {
    pub name: String,
    pub pattern: Regex,
    pub methods: Vec<String>,
    pub upstream: String,
    pub strip_prefix: bool,
    pub middlewares: Vec<String>,
}

impl CompiledRoute {
    pub fn from_config(name: String, cfg: RouteConfig) -> ProxyResult<Self> {
        let pattern = Regex::new(&cfg.path_pattern)
            .map_err(|e| ProxyError::InvalidRegex { pattern: cfg.path_pattern, source: e })?;
        Ok(CompiledRoute {
            name,
            pattern,
            methods: cfg.methods,
            upstream: cfg.upstream,
            strip_prefix: cfg.strip_prefix.unwrap_or(false),
            middlewares: cfg.middlewares,
        })
    }
}

pub fn load() -> ProxyResult<UnifiedConfig> {
    dotenv().ok();
    let args = CliArgs::parse();
    let config_path = args.config.unwrap_or_else(|| "config.yaml".into());
    let config_content = fs::read_to_string(&config_path)?;
    let mut config: UnifiedConfig = serde_yaml::from_str(&config_content)?;
    if let Some(mode_str) = args.mode {
        config.mode = match mode_str.as_str() {
            "load_balancer" => OperationMode::LoadBalancer,
            "api_gateway" => OperationMode::ApiGateway,
            _ => unreachable!(),
        };
    }
    if let Some(bind) = args.bind { config.server.bind_address = bind; }
    if let Some(workers) = args.workers { config.server.workers = Some(workers); }
    match config.mode {
        OperationMode::LoadBalancer => {
            if config.load_balancer.is_none() {
                return Err(ProxyError::MissingField("load_balancer".into()));
            }
        }
        OperationMode::ApiGateway => {
            if config.api_gateway.is_none() {
                return Err(ProxyError::MissingField("api_gateway".into()));
            }
        }
    }
    for (name, _) in &config.upstreams {
        if name.is_empty() { return Err(ProxyError::Config("Upstream name cannot be empty".into())); }
    }
    Ok(config)
}
