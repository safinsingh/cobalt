use crate::db;
use crate::db::query::LatestServiceMap;
use axum::extract::State;
use axum::http::StatusCode;
use axum::response::IntoResponse;
use axum::response::Response;
use axum::Json;
use axum::{routing::get, Router};
use sqlx::PgPool;
use tokio::net::TcpListener;

struct WebError(anyhow::Error);
type WebResult<T> = Result<T, WebError>;

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

async fn service_statuses(State(pool): State<PgPool>) -> WebResult<Json<Vec<LatestServiceMap>>> {
	let teams = db::query::latest_service_statuses(&pool).await?;
	Ok(Json(teams))
}

pub async fn run(port: u16, pool: PgPool) -> anyhow::Result<()> {
	let app = Router::new()
		.route("/service_statuses", get(service_statuses))
		.with_state(pool);
	let listener = TcpListener::bind(("0.0.0.0", port)).await?;

	axum::serve(listener, app).await?;
	Ok(())
}
