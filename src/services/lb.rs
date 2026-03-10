use std::sync::Arc;
use async_trait::async_trait;
use pingora::proxy::{ProxyHttp, Session};
use pingora::upstreams::peer::HttpPeer;
use pingora_load_balancing::{LoadBalancer, selection::RoundRobin};

pub struct LoadBalancerService { upstream: Arc<LoadBalancer<RoundRobin>> }
impl LoadBalancerService {
    pub fn new(upstream: Arc<LoadBalancer<RoundRobin>>) -> Self { Self { upstream } }
}
#[async_trait]
impl ProxyHttp for LoadBalancerService {
    type CTX = ();
    fn new_ctx(&self) -> Self::CTX {}
    async fn upstream_peer(&self, _session: &mut Session, _ctx: &mut Self::CTX) -> pingora::Result<Option<Box<HttpPeer>>> {
        Ok(Some(self.upstream.select(b"", 256)?))
    }
}
