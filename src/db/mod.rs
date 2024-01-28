mod models;
mod query;

pub struct Db {
	conn: sqlx::pool::Pool<sqlx::Postgres>,
}
