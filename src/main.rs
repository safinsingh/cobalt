mod checks;
mod config;
mod db;
mod offset;
mod score;
mod shuffle;

use dotenvy::dotenv;

use crate::config::Config;
use std::fs;
use std::sync::Arc;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
	dotenv()?;
	let raw = fs::read_to_string("cobalt.yml")?;
	let cfg = Config::from_str(&raw)?;
	let pool = db::establish_pg_conn().await?;

	let scoring = Arc::new(false);
	while *scoring {
		score::run(&cfg, &pool).await?;
		tokio::time::sleep(cfg.timing.jittered_interval().to_std()?);
	}

	Ok(())
}
