use crate::auth::AuthSession;
use askama_axum::IntoResponse;
use axum::{http::StatusCode, response::Redirect};

pub async fn get(mut auth_session: AuthSession) -> impl IntoResponse {
	match auth_session.logout().await {
		Ok(_) => Redirect::to("/login").into_response(),
		Err(_) => StatusCode::INTERNAL_SERVER_ERROR.into_response(),
	}
}
