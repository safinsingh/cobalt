mod checks;
mod config;
mod db;
mod offset;
mod score;
mod shuffle;
mod web;

use crate::config::Config;
use dotenvy::dotenv;
use log::{debug, LevelFilter};
use std::{fs, sync::Arc};
use tokio::task;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
	env_logger::builder().filter_level(LevelFilter::Info).init();
	dotenv()?;

	let raw = fs::read_to_string("cobalt.yml")?;
	let cfg = Arc::new(Config::from_str(&raw)?);
	debug!("Parsed configuration: {:#?}", cfg);

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
