use sqlx::{Pool, Postgres, Transaction};

pub mod models;
pub mod query;

pub type PgPool = Pool<Postgres>;
pub type PgTransaction<'a> = Transaction<'a, Postgres>;
