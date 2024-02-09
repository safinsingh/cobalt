mod injects;
mod login;
mod logout;
mod status;

use crate::{auth::AuthSession, config::Config};
use axum::{
	http::StatusCode,
	response::{IntoResponse, Response},
	routing::{get, post},
	Router,
};
use axum_login::{
	login_required, tower_sessions::ExpiredDeletion, AuthManagerLayerBuilder, AuthnBackend,
};
use axum_messages::MessagesManagerLayer;
use log::info;
use sqlx::PgPool;
use std::sync::Arc;
use tokio::{net::TcpListener, signal, sync::RwLock, task::AbortHandle};
use tower_http::services::ServeDir;
use tower_sessions::{Expiry, SessionManagerLayer};
use tower_sessions_sqlx_store::PostgresStore;

#[derive(Clone)]
pub struct WebState {
	config: Config,
	pool: PgPool,
	scoring: Arc<RwLock<bool>>,
}

impl WebState {
	pub async fn is_running(&self) -> bool {
		*self.scoring.read().await
	}
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

pub struct BaseTemplate {
	pub mock_title: String,
	pub user: Option<<Config as AuthnBackend>::User>,
	pub running: bool,
}

impl BaseTemplate {
	pub async fn from_params(state: &WebState, auth_session: AuthSession) -> Self {
		Self {
			mock_title: state.config.round.clone(),
			running: state.is_running().await,
			user: auth_session.user,
		}
	}

	fn team_name(&self) -> &str {
		&self.user.as_ref().unwrap().username
	}
}

pub async fn run(config: Config, pool: PgPool, scoring: Arc<RwLock<bool>>) -> anyhow::Result<()> {
	let session_store = PostgresStore::new(pool.clone());
	session_store.migrate().await?;

	// taken from `axum-login` example
	let deletion_task = tokio::task::spawn(
		session_store
			.clone()
			.continuously_delete_expired(tokio::time::Duration::from_secs(60)),
	);

	let session_layer = SessionManagerLayer::new(session_store)
		.with_secure(false)
		.with_expiry(Expiry::OnInactivity(time::Duration::days(1)));
	let auth_layer = AuthManagerLayerBuilder::new(config.clone(), session_layer).build();

	let protected = Router::new()
		.route("/injects", get(injects::get))
		.route("/injects/:inject_number", get(injects::page::get))
		.route_layer(login_required!(Config, login_url = "/login"))
		.route("/login", get(login::get))
		.route("/login", post(login::post))
		.route("/logout", get(logout::get));

	let app = Router::new()
		.route("/", get(status::get))
		.nest_service("/assets", ServeDir::new("assets"))
		.nest_service(
			"/injects/sources",
			ServeDir::new(&config.inject_meta.source_dir),
		)
		.nest_service(
			"/injects/assets",
			ServeDir::new(&config.inject_meta.assets_dir),
		)
		.merge(protected)
		.layer(MessagesManagerLayer)
		.layer(auth_layer)
		.with_state(WebState {
			config: config.clone(),
			pool,
			scoring,
		});

	let listener = TcpListener::bind(("0.0.0.0", config.web.port)).await?;
	info!("Web server running on {}", listener.local_addr()?);
	axum::serve(listener, app)
		.with_graceful_shutdown(shutdown_signal(deletion_task.abort_handle()))
		.await?;

	deletion_task.await??;
	Ok(())
}

// taken from `axum-login` example
async fn shutdown_signal(deletion_task_abort_handle: AbortHandle) {
	let ctrl_c = async {
		signal::ctrl_c()
			.await
			.expect("failed to install Ctrl+C handler");
	};

	#[cfg(unix)]
	let terminate = async {
		signal::unix::signal(signal::unix::SignalKind::terminate())
			.expect("failed to install signal handler")
			.recv()
			.await;
	};

	#[cfg(not(unix))]
	let terminate = std::future::pending::<()>();

	tokio::select! {
		_ = ctrl_c => { deletion_task_abort_handle.abort() },
		_ = terminate => { deletion_task_abort_handle.abort() },
	}
}
