use std::net::Ipv4Addr;

use crate::{
	checks::{Check, CheckResult},
	config::{check_types::Ssh, Vm},
};
use async_trait::async_trait;

#[async_trait]
impl Check for Ssh {
	async fn score(&self, ip: Ipv4Addr, vm: &Vm) -> CheckResult {
		Ok(())
	}
}
