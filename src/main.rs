mod checks;
mod config;
mod db;
mod offset;
mod score;
mod shuffle;

use crate::config::Config;
use std::fs;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
	let raw = fs::read_to_string("cobalt.yml")?;
	let cfg = Config::from_str(&raw)?;

	println!("{:?}", cfg);

	Ok(())
}
