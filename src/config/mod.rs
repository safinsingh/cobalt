pub mod check_types;

use crate::checks::{Check, CheckResult};
use anyhow::{bail, ensure};
use enum_dispatch::enum_dispatch;
use rand::Rng;
use serde::Deserialize;
use std::{collections::HashMap, net::Ipv4Addr, ops::Deref, path::PathBuf, sync::Arc};

// check interval (default: 120sec)
const DEFAULT_INTERVAL: u32 = 120;
// check jitter min/max (default: 10sec)
const DEFAULT_JITTER: u32 = 10;
// check timeout (default: 30sec)
const DEFAULT_CHECK_TIMEOUT: u32 = 30;

// most consecutive downs before SLA is triggered
const DEFAULT_MAX_CONSECUTIVE_DOWNS: u32 = 5;

/// SCORING ///
// service up point differential (default: +5)
const DEFAULT_SERVICE_UP_POINTS: i32 = 5;
// service down point differential (default: +0)
const DEFAULT_SERVICE_DOWN_POINTS: i32 = 0;
// sla point differential (default: -15)
const DEFAULT_SLA_POINTS: i32 = -15;

/// WEB ///
// default web interface port
const DEFAULT_WEB_PORT: u16 = 8080;

#[derive(Deserialize, Debug)]
#[serde(tag = "type")]
#[serde(rename_all = "snake_case")]
#[enum_dispatch]
pub enum Service {
	Http(check_types::Http),
	Ssh(check_types::Ssh),
}

#[derive(Deserialize, Debug)]
pub struct Vm {
	pub ip: u8,
	pub services: HashMap<String, Service>,
}

#[derive(Deserialize, Debug)]
#[serde(tag = "type")]
#[serde(rename_all = "snake_case")]
pub enum InjectType {
	Service {
		vm: String,
		services: HashMap<String, Service>,
	},
	Response,
}

impl std::fmt::Display for InjectType {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		match self {
			Self::Service { vm, .. } => write!(f, "Service (vm='{}')", vm),
			Self::Response => write!(f, "Response"),
		}
	}
}

#[derive(Deserialize, Debug)]
pub struct Inject {
	pub title: String,
	pub source: PathBuf,
	pub offset: crate::offset::Offset,
	#[serde(flatten)]
	pub inner: InjectType,
}

fn default_max_consecutive_downs() -> u32 {
	DEFAULT_MAX_CONSECUTIVE_DOWNS
}

#[derive(Deserialize, Debug, Default)]
pub struct Slas {
	pub enable: bool,
	#[serde(default = "default_max_consecutive_downs")]
	pub max_consecutive_downs: u32,
}

fn default_service_up_points() -> i32 {
	DEFAULT_SERVICE_UP_POINTS
}

fn default_service_down_points() -> i32 {
	DEFAULT_SERVICE_DOWN_POINTS
}

fn default_sla_points() -> i32 {
	DEFAULT_SLA_POINTS
}

#[derive(Deserialize, Debug, Default, Clone, Copy)]
pub struct Scoring {
	#[serde(default = "default_service_up_points")]
	pub service_up: i32,
	#[serde(default = "default_service_down_points")]
	pub service_down: i32,
	#[serde(default = "default_sla_points")]
	pub sla: i32,
}

fn default_interval() -> u32 {
	DEFAULT_INTERVAL
}
fn default_jitter() -> u32 {
	DEFAULT_JITTER
}
fn default_check_timeout() -> u32 {
	DEFAULT_CHECK_TIMEOUT
}

#[derive(Deserialize, Debug)]
pub struct Timing {
	#[serde(default = "default_interval")]
	pub interval: u32,
	#[serde(default = "default_jitter")]
	pub jitter: u32,
	#[serde(default = "default_check_timeout")]
	pub check_timeout: u32,
}

impl Timing {
	pub fn jittered_interval(&self) -> std::time::Duration {
		let offset = rand::thread_rng().gen_range(-1 * self.jitter as i32..self.jitter as i32);
		std::time::Duration::from_secs(self.interval as u64 + offset as u64)
	}
}

fn default_web_port() -> u16 {
	DEFAULT_WEB_PORT
}

#[derive(Deserialize, Debug)]
pub struct Web {
	pub admin_username: String,
	pub admin_password: String,
	#[serde(default = "default_web_port")]
	pub port: u16,
}

#[derive(Deserialize, Debug)]
pub struct Team {
	pub subnet: String,
	pub password: String,
}

#[derive(Deserialize, Debug)]
pub struct ConfigInner {
	pub round: String,
	pub inject_dir: PathBuf,
	pub timing: Timing,
	pub scoring: Scoring,
	pub web: Web,
	#[serde(default)]
	pub slas: Slas,
	// more intuitive naming
	pub vms: HashMap<String, Vm>,
	pub injects: Vec<Inject>,
	pub teams: HashMap<String, Team>,
}

#[derive(Clone)]
pub struct Config {
	inner: Arc<ConfigInner>,
}

impl Deref for Config {
	type Target = ConfigInner;

	fn deref(&self) -> &Self::Target {
		&self.inner
	}
}

impl std::fmt::Debug for Config {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		std::fmt::Debug::fmt(&self.inner, f)
	}
}

impl Config {
	pub fn from_str(s: &str) -> anyhow::Result<Self> {
		let cfg: ConfigInner = serde_yaml::from_str(s)?;
		cfg.validate()?;
		Ok(Self {
			inner: Arc::new(cfg),
		})
	}
}

impl ConfigInner {
	fn validate(&self) -> anyhow::Result<()> {
		self.validate_teams()?;
		self.validate_injects()
	}

	fn validate_teams(&self) -> anyhow::Result<()> {
		for (team_alias, team) in &self.teams {
			let ip_str = team.subnet.replace('x', "1");
			if ip_str.parse::<Ipv4Addr>().is_err() {
				bail!("Invalid subnet for team '{}': {}", team_alias, team.subnet);
			}
		}
		Ok(())
	}

	fn validate_injects(&self) -> anyhow::Result<()> {
		for inject in &self.injects {
			if let InjectType::Service { vm, services } = &inject.inner {
				if let Some(existing_vm) = self.vms.get(vm) {
					for service_alias in services.keys() {
						ensure!(
							!existing_vm.services.contains_key(service_alias),
							"inject '{}' creates service '{}' on vm '{}', but a service with that name already exists",
							inject.title, service_alias, vm
						);
					}
				} else {
					bail!("inject '{}' refers to unknown vm '{}'", inject.title, vm);
				}
			}
		}

		Ok(())
	}
}
