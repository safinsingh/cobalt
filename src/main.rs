mod checks;
mod config;
mod db;
mod offset;
mod score;
mod shuffle;

use dotenvy::dotenv;

use crate::config::Config;
use std::fs;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
	dotenv()?;
	let raw = fs::read_to_string("cobalt.yml")?;
	let cfg = Config::from_str(&raw)?;
	let pool = db::establish_pg_conn().await?;

	cfg.score(pool).await?;

	Ok(())
}
