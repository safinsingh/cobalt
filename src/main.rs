mod auth;
mod checks;
mod config;
mod db;
mod offset;
mod score;
mod shuffle;
mod state;
mod web;

use crate::{config::Config, state::Timer};
use dotenvy::dotenv;
use log::{debug, LevelFilter};
use std::fs;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
	env_logger::builder().filter_level(LevelFilter::Info).init();
	dotenv()?;

	let raw = fs::read_to_string("cobalt.yml")?;
	let cfg = Config::from_str(&raw)?;
	debug!("Parsed configuration: {:#?}", cfg);
	let pool = db::establish_pg_conn().await?;
	let timer = Timer::default();

	tokio::spawn(score::run(cfg.clone(), timer.clone(), pool.clone()));
	web::run(cfg, timer, pool).await
}
