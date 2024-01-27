use regex::Regex;
use serde::Deserialize;
use serde_with::{serde_as, DisplayFromStr};
use std::collections::HashMap;

#[serde_as]
#[derive(Deserialize, Debug)]
pub struct HttpInner {
	#[serde_as(as = "DisplayFromStr")]
	pub method: reqwest::Method,
	pub path: String,
	pub headers: Option<HashMap<String, String>>,
	pub body: Option<String>,
	pub contains: Option<String>,
	#[serde(with = "serde_regex")]
	pub contains_regex: Option<Regex>,
}

#[derive(Deserialize, Debug)]
pub struct Http {
	#[serde(flatten)]
	pub pages: Vec<HttpInner>,
}

#[derive(Deserialize, Debug)]
#[serde(tag = "login")]
#[serde(rename_all = "snake_case")]
pub enum SshLoginType {
	Unix { user: String },
	Custom { user: String, password: String },
	None,
}

#[derive(Deserialize, Debug)]
pub struct Ssh {
	#[serde(flatten)]
	inner: SshLoginType,
}
