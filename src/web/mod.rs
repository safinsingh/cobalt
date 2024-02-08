mod auth;
mod injects;
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
use tokio::{net::TcpListener, signal, task::AbortHandle};
use tower_http::services::ServeDir;
use tower_sessions::{Expiry, SessionManagerLayer};
use tower_sessions_sqlx_store::PostgresStore;

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

pub struct BaseTemplate {
	pub mock_title: String,
	pub user: Option<<Config as AuthnBackend>::User>,
}

impl BaseTemplate {
	fn from_params(config: Config, auth_session: AuthSession) -> Self {
		Self {
			mock_title: config.round.clone(),
			user: auth_session.user,
		}
	}

	fn get_team<'a>(&'a self) -> &'a str {
		&self.user.as_ref().unwrap().username
	}
}

pub async fn run(config: Config, pool: PgPool) -> anyhow::Result<()> {
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

	let listener = TcpListener::bind(("0.0.0.0", config.web.port)).await?;

	let protected = Router::new()
		.route("/injects", get(injects::get))
		.route_layer(login_required!(Config, login_url = "/login"))
		.route("/login", get(auth::login::get))
		.route("/login", post(auth::login::post))
		.route("/logout", get(auth::logout::get));

	let app = Router::new()
		.route("/status", get(status::get))
		.nest_service("/assets", ServeDir::new("assets"))
		.merge(protected)
		.layer(MessagesManagerLayer)
		.layer(auth_layer)
		.with_state(WebState { pool, config });

	info!("Web server running on {}", listener.local_addr()?);
	axum::serve(listener, app)
		// .with_graceful_shutdown(shutdown_signal(deletion_task.abort_handle()))
		.await?;
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
