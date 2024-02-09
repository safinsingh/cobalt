use regex::Regex;
use serde::Deserialize;
use serde_with::{serde_as, DisplayFromStr};
use std::{collections::HashMap, path::PathBuf};

fn default_http_method() -> reqwest::Method {
	reqwest::Method::GET
}

#[serde_as]
#[derive(Deserialize, Debug)]
pub struct HttpInner {
	#[serde(default = "default_http_method")]
	#[serde_as(as = "DisplayFromStr")]
	pub method: reqwest::Method,
	pub path: String,
	pub headers: Option<HashMap<String, String>>,
	pub body: Option<String>,
	pub contains: Option<String>,
	#[serde(default)]
	#[serde(with = "serde_regex")]
	pub contains_regex: Option<Regex>,
}

#[derive(Deserialize, Debug)]
pub struct Http {
	pub pages: Vec<HttpInner>,
}

#[derive(Deserialize, Debug)]
#[serde(tag = "method")]
#[serde(rename_all = "snake_case")]
pub enum SshAuthType {
	Password {
		user: String,
		password: String,
	},
	Pubkey {
		user: String,
		private_key: PathBuf,
		passphrase: Option<String>,
	},
}

fn default_ssh_port() -> u16 {
	22
}

#[derive(Deserialize, Debug)]
pub struct Ssh {
	#[serde(default = "default_ssh_port")]
	pub port: u16,
	pub auth: SshAuthType,
	pub command: Option<String>,
}
