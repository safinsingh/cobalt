pub mod check_types;

use crate::checks::{Check, CheckResult};
use anyhow::bail;
use enum_dispatch::enum_dispatch;
use rand::Rng;
use serde::Deserialize;
use std::{collections::HashMap, net::Ipv4Addr, path::PathBuf};

// check interval (default: 120sec)
const DEFAULT_INTERVAL: u32 = 120;
// check jitter min/max (default: 10sec)
const DEFAULT_JITTER: u32 = 10;
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
	pub credentials: HashMap<String, String>,
}

#[derive(Deserialize, Debug)]
#[serde(tag = "type")]
#[serde(rename_all = "snake_case")]
enum InjectType {
	Service {
		#[serde(rename = "box")]
		vm: String,
		services: HashMap<String, Service>,
	},
	Response,
}

#[derive(Deserialize, Debug)]
pub struct Inject {
	title: String,
	source: PathBuf,
	offset: crate::offset::Offset,
	#[serde(flatten)]
	inner: InjectType,
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

#[derive(Deserialize, Debug)]
pub struct Timing {
	#[serde(default = "default_interval")]
	pub interval: u32,
	#[serde(default = "default_jitter")]
	pub jitter: u32,
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
pub struct Config {
	pub round: String,
	pub inject_dir: PathBuf,
	pub timing: Timing,
	pub scoring: Scoring,
	pub web: Web,
	#[serde(default)]
	pub slas: Slas,
	// more intuitive naming
	#[serde(rename = "boxes")]
	pub vms: HashMap<String, Vm>,
	pub injects: Vec<Inject>,
	pub teams: HashMap<String, String>,
}

impl Config {
	pub fn from_str(s: &str) -> anyhow::Result<Self> {
		let cfg: Self = serde_yaml::from_str(s)?;
		cfg.validate()?;
		Ok(cfg)
	}

	fn validate(&self) -> anyhow::Result<()> {
		self.validate_teams()
	}

	fn validate_teams(&self) -> anyhow::Result<()> {
		for (alias, subnet) in &self.teams {
			let ip_str = subnet.replace('x', "1");
			if ip_str.parse::<Ipv4Addr>().is_err() {
				bail!("Invalid subnet for team '{}': {}", alias, subnet);
			}
		}
		Ok(())
	}
}
