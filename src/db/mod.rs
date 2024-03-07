pub mod models;
pub mod mutation;
pub mod query;

use anyhow::Context;
use log::LevelFilter;
use sqlx::{
	postgres::{PgConnectOptions, PgPoolOptions},
	ConnectOptions, PgPool,
};
use std::env;
use url::Url;

const DB_URL_ENV_VAR: &str = "DATABASE_URL";

pub async fn establish_pg_conn() -> anyhow::Result<PgPool> {
	let url = Url::parse(&env::var(DB_URL_ENV_VAR).context(DB_URL_ENV_VAR)?)?;
	let pool = PgPoolOptions::new()
		.connect_with(PgConnectOptions::from_url(&url)?.log_statements(LevelFilter::Debug))
		.await?;
	Ok(pool)
}
