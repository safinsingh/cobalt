use std::net::Ipv4Addr;

use crate::checks::Check;
use crate::config::check_types::Ssh;
use crate::config::Config;
use async_trait::async_trait;

#[async_trait]
impl Check for Ssh {
	async fn score(&self, ip: Ipv4Addr) -> anyhow::Result<()> {
		Ok(())
	}
}
