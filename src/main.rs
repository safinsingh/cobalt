mod checks;
mod config;
mod db;
mod offset;
mod score;
mod shuffle;
mod web;

use dotenvy::dotenv;
use tokio::task;

use crate::config::Config;
use std::fs;
use std::sync::Arc;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
	dotenv()?;
	let raw = fs::read_to_string("cobalt.yml")?;
	let cfg = Arc::new(Config::from_str(&raw)?);
	let pool = db::establish_pg_conn().await?;
	let is_scoring = false;

	web::run(cfg.clone(), pool.clone()).await?;
	Ok(())
	// loop {
	// 	if is_scoring {
	// 		score::run(&cfg, pool.clone()).await.unwrap();
	// 		tokio::time::sleep(cfg.timing.jittered_interval()).await;
	// 	}
	// }
}
