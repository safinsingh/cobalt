use crate::checks::Check;
use anyhow::bail;
use enum_dispatch::enum_dispatch;
use serde::Deserialize;
use std::collections::HashMap;
use std::net::Ipv4Addr;
use std::path::PathBuf;

// check interval (default: 120sec)
const DEFAULT_INTERVAL: u32 = 120;
// check jitter min/max (default: 10sec)
const DEFAULT_JITTER: u32 = 10;

pub mod check_types {
	use serde::Deserialize;
	use serde_with::{serde_as, DisplayFromStr};
	use std::collections::HashMap;

	#[derive(Deserialize, Debug)]
	#[serde(tag = "login")]
	#[serde(rename_all = "snake_case")]
	pub enum SshLoginType {
		Unix { user: String },
		Custom { user: String, password: String },
		None,
	}

	#[serde_as]
	#[derive(Deserialize, Debug)]
	pub struct HttpInner {
		#[serde_as(as = "DisplayFromStr")]
		pub method: reqwest::Method,
		pub path: String,
		pub headers: Option<HashMap<String, String>>,
		pub body: Option<String>,
	}

	#[derive(Deserialize, Debug)]
	pub struct Http {
		#[serde(flatten)]
		pub pages: Vec<HttpInner>,
	}

	#[derive(Deserialize, Debug)]
	pub struct HttpContent {
		path: String,
		content: String,
	}

	#[derive(Deserialize, Debug)]
	pub struct Ssh {
		#[serde(flatten)]
		inner: SshLoginType,
	}
}

#[derive(Deserialize, Debug)]
#[serde(tag = "type")]
#[serde(rename_all = "snake_case")]
#[enum_dispatch]
pub enum Service {
	Http(check_types::Http),
	HttpContent(check_types::HttpContent),
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

fn default_interval() -> u32 {
	DEFAULT_INTERVAL
}
fn default_jitter() -> u32 {
	DEFAULT_JITTER
}

#[derive(Deserialize, Debug)]
pub struct Config {
	pub round: String,
	pub inject_dir: PathBuf,
	#[serde(default = "default_interval")]
	pub interval: u32,
	#[serde(default = "default_jitter")]
	pub jitter: u32,
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
