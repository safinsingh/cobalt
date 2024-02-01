use std::env;

use anyhow::Context;
use sqlx::{pool::PoolOptions, PgPool};

pub mod models;
pub mod query;

const DB_URL_ENV_VAR: &str = "DATABASE_URL";

pub async fn establish_pg_conn() -> anyhow::Result<PgPool> {
	let url = env::var(DB_URL_ENV_VAR).context("db url in .env")?;
	let pool = PoolOptions::default().connect(&url).await?;
	Ok(pool)
}
