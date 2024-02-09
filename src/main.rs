mod auth;
mod checks;
mod config;
mod db;
mod offset;
mod score;
mod shuffle;
mod time;
mod web;

use crate::config::Config;
use dotenvy::dotenv;
use log::{debug, LevelFilter};
use std::{fs, sync::Arc};
use tokio::sync::RwLock;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
	env_logger::builder().filter_level(LevelFilter::Info).init();
	dotenv()?;

	let raw = fs::read_to_string("cobalt.yml")?;
	let cfg = Config::from_str(&raw)?;
	debug!("Parsed configuration: {:#?}", cfg);
	let pool = db::establish_pg_conn().await?;

	let running = Arc::new(RwLock::new(false));
	tokio::spawn(score::run(cfg.clone(), pool.clone(), running.clone()));
	web::run(cfg, pool, running).await
}
