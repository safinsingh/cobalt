pub mod errors;
mod http;
mod ssh;

use crate::config::Vm;
use async_trait::async_trait;
use enum_dispatch::enum_dispatch;
use std::net::Ipv4Addr;

pub(crate) use errors::check_bail;
pub use errors::{CheckError, CheckResult};

#[async_trait]
#[enum_dispatch(Service)]
pub trait Check {
	async fn score(&self, ip: Ipv4Addr, vm: &Vm) -> CheckResult;
}
