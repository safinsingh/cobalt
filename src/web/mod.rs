use crate::config::Config;
use crate::db;
use crate::db::models::ServiceGatheredInfo;
use crate::db::query::LatestTeamSnapshot;
use crate::db::query::OwnedServiceMap;
use askama_axum::Template;
use axum::extract::State;
use axum::http::StatusCode;
use axum::response::Html;
use axum::response::IntoResponse;
use axum::response::Response;
use axum::Json;
use axum::{routing::get, Router};
use chrono::{DateTime, Utc};
use sqlx::PgPool;
use std::collections::HashMap;
use std::sync::Arc;
use tokio::net::TcpListener;

#[derive(Clone)]
struct WebState {
	pool: PgPool,
	config: Arc<Config>,
}

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

#[derive(Template)]
#[template(path = "service_statuses.html")]
struct ServiceStatusTpl {
	// service names altered to be <vm>-<service>
	status_table: OwnedServiceMap,
	service_list: Vec<String>,
	latest_time: DateTime<Utc>,
}

type FlattenedServiceMap = HashMap<String, HashMap<String, ServiceGatheredInfo>>;
fn flatten_team_snapshots(
	snapshots: Vec<LatestTeamSnapshot>,
) -> (FlattenedServiceMap, Vec<String>) {
	let mut result_map: FlattenedServiceMap = HashMap::new();
	let mut service_list = Vec::new();
	for snapshot in snapshots.into_iter() {
		let mut service_flat_map = HashMap::new();
		for (vm, services) in snapshot.services.0.into_iter() {
			for (service, info) in services {
				let key = format!("{}-{}", vm, service);
				service_list.push(key.clone());
				service_flat_map.insert(key, info);
			}
		}
		result_map.insert(snapshot.team, service_flat_map);
	}
	(result_map, service_list)
}

async fn service_statuses(State(ctxt): State<WebState>) -> WebResult<impl IntoResponse> {
	let teams = db::query::latest_service_statuses(&ctxt.pool).await?;
	let latest_time = teams.iter().map(|t| t.time).max().unwrap_or_default();
	let (status_table, service_list) = flatten_team_snapshots(teams);

	Ok(ServiceStatusTpl {
		status_table,
		service_list,
		latest_time,
	})
}

pub async fn run(config: Arc<Config>, pool: PgPool) -> anyhow::Result<()> {
	let app = Router::new()
		.route("/service_statuses", get(service_statuses))
		.with_state(WebState {
			pool,
			config: config.clone(),
		});
	let listener = TcpListener::bind(("0.0.0.0", config.web.port)).await?;

	axum::serve(listener, app).await?;
	Ok(())
}
