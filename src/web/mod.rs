mod login;
mod status;

use crate::config::Config;
use axum::{
	http::StatusCode,
	response::{IntoResponse, Response},
	routing::get,
	Router,
};
use itertools::Itertools;
use log::info;
use sqlx::PgPool;
use std::{net::SocketAddrV4, sync::Arc};
use tokio::net::TcpListener;
use tower_http::services::ServeDir;

#[derive(Clone)]
pub struct WebState {
	pool: PgPool,
	config: Config,
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

impl WebState {
	fn title(&self) -> String {
		self.config.round.to_owned()
	}
}

pub async fn run(config: Config, pool: PgPool) -> anyhow::Result<()> {
	let listener = TcpListener::bind(("0.0.0.0", config.web.port)).await?;
	let app = Router::new()
		.route("/login", get(login::login))
		.route("/status", get(status::status))
		.nest_service("/assets", ServeDir::new("assets"))
		.with_state(WebState { pool, config });

	info!("Web server running on {}", listener.local_addr()?);
	axum::serve(listener, app).await?;
	Ok(())
}
