use serde::Deserialize;
use std::fs;

#[derive(Deserialize, Debug)]
#[serde(tag = "type")]
#[serde(rename_all = "snake_case")]
enum ServiceTypes {
    Http { path: String },
    HttpContent { path: String, content: String },
    Ssh,
}

#[derive(Deserialize, Debug)]
struct Service {
    alias: String,
    #[serde(flatten)]
    inner: ServiceTypes,
}

#[derive(Deserialize, Debug)]
struct Box {
    name: String,
    ip: u8,
    services: Vec<Service>,
}

#[derive(Deserialize, Debug)]
struct Config {
    round: String,
    inject_dir: String,
    boxes: Vec<Box>,
    subnets: Vec<String>,
}

fn main() -> anyhow::Result<()> {
    let raw = fs::read_to_string("cobalt.yml")?;
    let cfg: Config = serde_yaml::from_str(&raw)?;

    println!("{:?}", cfg);

    Ok(())
}
