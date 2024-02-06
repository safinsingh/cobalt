mod routes;

use crate::config::Config;
use axum::{
	http::StatusCode,
	response::{IntoResponse, Response},
	routing::get,
	Router,
};
use itertools::Itertools;
use sqlx::PgPool;
use std::sync::Arc;
use tokio::net::TcpListener;
use tower_http::services::ServeDir;

#[derive(Clone)]
pub struct WebState {
	pool: PgPool,
	config: Arc<Config>,
}

pub struct WebError(anyhow::Error);
pub type WebResult<T> = Result<T, WebError>;

impl IntoResponse for WebError {
	fn into_response(self) -> Response {
		(
			StatusCode::INTERNAL_SERVER_ERROR,
			format!("Internal Server Error: {}", self.0),
		)
			.into_response()
	}
}

impl<E> From<E> for WebError
where
	E: Into<anyhow::Error>,
{
	fn from(err: E) -> Self {
		Self(err.into())
	}
}

pub async fn run(config: Arc<Config>, pool: PgPool) -> anyhow::Result<()> {
	let listener = TcpListener::bind(("0.0.0.0", config.web.port)).await?;
	let app = Router::new()
		.route("/status", get(routes::status))
		.nest_service("/assets", ServeDir::new("assets"))
		.with_state(WebState {
			pool,
			config: config.clone(),
		});

	axum::serve(listener, app).await?;
	Ok(())
}
