use std::collections::HashMap;
use std::sync::Arc;
use async_trait::async_trait;
use http::Method;
use pingora::proxy::{ProxyHttp, Session};
use pingora::upstreams::peer::HttpPeer;
use pingora_load_balancing::{LoadBalancer, selection::RoundRobin};
use tracing::warn;
use crate::config::CompiledRoute;

pub struct ApiGatewayService {
    routes: Vec<CompiledRoute>,
    upstream_pools: HashMap<String, Arc<LoadBalancer<RoundRobin>>>,
}
impl ApiGatewayService {
    pub fn new(routes: Vec<CompiledRoute>, upstream_pools: HashMap<String, Arc<LoadBalancer<RoundRobin>>>) -> Self {
        Self { routes, upstream_pools }
    }
    fn find_route(&self, path: &str, method: &Method) -> Option<&CompiledRoute> {
        for route in &self.routes {
            if route.pattern.is_match(path) {
                if !route.methods.is_empty() {
                    let method_str = method.as_str();
                    if !route.methods.iter().any(|m| m == method_str) {
                        continue;
                    }
                }
                return Some(route);
            }
        }
        None
    }
}
#[async_trait]
impl ProxyHttp for ApiGatewayService {
    type CTX = ();
    fn new_ctx(&self) -> Self::CTX {}
    async fn upstream_peer(&self, session: &mut Session, _ctx: &mut Self::CTX) -> pingora::Result<Option<Box<HttpPeer>>> {
        let req = session.req_header();
        let path = req.uri.path();
        let method = req.method.clone();
        if let Some(route) = self.find_route(path, &method) {
            if let Some(pool) = self.upstream_pools.get(&route.upstream) {
                return Ok(Some(pool.select(b"", 256)?));
            } else {
                warn!("Upstream '{}' not found for route {}", route.upstream, route.name);
            }
        }
        Ok(None)
    }
}
