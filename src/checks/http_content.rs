use crate::checks::Check;
use crate::config::check_types::HttpContent;
use async_trait::async_trait;
use std::net::Ipv4Addr;

#[async_trait]
impl Check for HttpContent {
	async fn score(&self, ip: Ipv4Addr) -> anyhow::Result<()> {
		Ok(())
	}
}
