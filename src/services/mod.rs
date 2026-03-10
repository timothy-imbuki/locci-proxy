use std::collections::HashMap;
use std::sync::Arc;
use std::time::Duration;
use pingora::proxy::http_proxy_service;
use pingora::services::Service;
use pingora_load_balancing::{LoadBalancer, health_check, selection::RoundRobin, Backend};
use tracing::info;
use crate::config::{UnifiedConfig, OperationMode, CompiledRoute};
use crate::errors::{ProxyResult, ProxyError};
mod lb;
mod gateway;
pub use lb::LoadBalancerService;
pub use gateway::ApiGatewayService;

pub fn build_service(config: &UnifiedConfig) -> ProxyResult<Box<dyn Service>> {
    let upstream_pools = build_upstream_pools(config)?;
    match config.mode {
        OperationMode::LoadBalancer => {
            let lb_config = config.load_balancer.as_ref().unwrap();
            let pool = upstream_pools.get(&lb_config.upstream)
                .ok_or_else(|| ProxyError::UpstreamNotFound(lb_config.upstream.clone()))?
                .clone();
            let service = LoadBalancerService::new(pool);
            Ok(Box::new(http_proxy_service(&pingora::server::configuration::ServerConf::default(), service)))
        }
        OperationMode::ApiGateway => {
            let gw_config = config.api_gateway.as_ref().unwrap();
            let mut routes = Vec::new();
            for (name, route_cfg) in &gw_config.routes {
                routes.push(CompiledRoute::from_config(name.clone(), route_cfg.clone())?);
            }
            routes.sort_by(|a, b| b.pattern.as_str().len().cmp(&a.pattern.as_str().len()));
            let service = ApiGatewayService::new(routes, upstream_pools);
            Ok(Box::new(http_proxy_service(&pingora::server::configuration::ServerConf::default(), service)))
        }
    }
}

fn build_upstream_pools(config: &UnifiedConfig) -> ProxyResult<HashMap<String, Arc<LoadBalancer<RoundRobin>>>> {
    let mut pools = HashMap::new();
    for (name, upstream_cfg) in &config.upstreams {
        if upstream_cfg.servers.is_empty() {
            return Err(ProxyError::EmptyUpstream(name.clone()));
        }
        let mut backends = Vec::new();
        for s in &upstream_cfg.servers {
            let (host, port_str) = s.split_once(':')
                .ok_or_else(|| ProxyError::Config(format!("Invalid server address: {}", s)))?;
            let port: u16 = port_str.parse().map_err(|_| ProxyError::Config(format!("Invalid port in {}", s)))?;
            backends.push(Backend { addr: format!("{}:{}", host, port) });
        }
        let mut lb = LoadBalancer::try_from_iter(backends).unwrap();
        if let Some(hc) = &upstream_cfg.health_check {
            let hc_path = hc.path.as_deref().unwrap_or("/");
            lb.set_health_check(health_check::HttpHealthCheck::new(hc_path));
            lb.health_check_frequency = Some(Duration::from_secs(hc.interval_secs));
        }
        pools.insert(name.clone(), Arc::new(lb));
        info!("Upstream '{}' configured with {} servers", name, upstream_cfg.servers.len());
    }
    Ok(pools)
}
