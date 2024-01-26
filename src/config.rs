use anyhow::bail;
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

    #[derive(Deserialize, Debug)]
    #[serde(tag = "login")]
    #[serde(rename_all = "snake_case")]
    pub enum SshLoginType {
        Unix { user: String },
        Custom { user: String, password: String },
        None,
    }

    #[derive(Deserialize, Debug)]
    struct HttpPage {
        method: reqwest::Method,
        path: String,
    }

    #[derive(Deserialize, Debug)]
    #[serde(untagged)]
    enum HttpInner {
        SinglePage(HttpPage),
        MultiPage(Vec<HttpPage>),
    }

    #[derive(Deserialize, Debug)]
    pub struct Http {
        #[serde(flatten)]
        inner: HttpInner,
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
pub enum Service {
    Http(check_types::Http),
    HttpContent(check_types::HttpContent),
    Ssh(check_types::Ssh),
}

#[derive(Deserialize, Debug)]
struct Box {
    ip: u8,
    services: HashMap<String, Service>,
}

#[derive(Deserialize, Debug)]
#[serde(tag = "type")]
#[serde(rename_all = "snake_case")]
enum InjectType {
    Service {
        r#box: String,
        services: HashMap<String, Service>,
    },
    Response,
}

#[derive(Deserialize, Debug)]
struct Inject {
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
    round: String,
    inject_dir: PathBuf,
    #[serde(default = "default_interval")]
    interval: u32,
    #[serde(default = "default_jitter")]
    jitter: u32,
    boxes: HashMap<String, Box>,
    injects: Vec<Inject>,
    teams: HashMap<String, String>,
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
