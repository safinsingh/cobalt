mod auth;
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
use std::fs;
use tokio::{sync::mpsc, task};

#[tokio::main]
async fn main() -> anyhow::Result<()> {
	env_logger::builder().filter_level(LevelFilter::Info).init();
	dotenv()?;

	let raw = fs::read_to_string("cobalt.yml")?;
	let cfg = Config::from_str(&raw)?;
	debug!("Parsed configuration: {:#?}", cfg);

	let pool = db::establish_pg_conn().await?;

	// score::run(&cfg, pool.clone()).await.unwrap();
	// score::run(&cfg, pool.clone()).await.unwrap();
	// score::run(&cfg, pool.clone()).await.unwrap();
	// score::run(&cfg, pool.clone()).await.unwrap();

	// let (tx, rx) = mpsc::channel::<()>(1);

	web::run(cfg.clone(), pool.clone()).await?;
	// task::spawn(async move {
	// 	loop {
	// 		rx.recv();
	// 		loop {
	// 			score::run(&cfg, pool.clone()).await.unwrap();
	// 			tokio::time::sleep(cfg.timing.jittered_interval()).await;
	// 		}
	// 	}
	// })
	// .await?;

	Ok(())
}
