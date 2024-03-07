use crate::{
	auth::{AuthSession, Credentials},
	get_base_template,
	state::EngineState,
	web::{BaseTemplate, WebCtxt, WebResult},
};
use askama::Template;
use askama_axum::IntoResponse;
use axum::{
	extract::{Query, State},
	http::StatusCode,
	response::Redirect,
	Form,
};
use axum_messages::{Message, Messages};
use serde::Deserialize;

// a majority of the following is taken from the axum-login example

#[derive(Debug, Deserialize)]
pub struct NextUrl {
	next: Option<String>,
}

#[derive(Template)]
#[template(path = "login.html")]
struct LoginTemplate {
	base: BaseTemplate,
	messages: Vec<Message>,
	next: Option<String>,
}

pub async fn get(
	State(ctxt): State<WebCtxt>,
	auth_session: AuthSession,
	messages: Messages,
	Query(NextUrl { next }): Query<NextUrl>,
) -> WebResult<impl IntoResponse> {
	Ok(LoginTemplate {
		base: get_base_template!(ctxt, auth_session),
		messages: messages.into_iter().collect(),
		next,
	})
}

pub async fn post(
	mut auth_session: AuthSession,
	messages: Messages,
	Form(creds): Form<Credentials>,
) -> impl IntoResponse {
	let team = match auth_session.authenticate(creds.clone()).await {
		Ok(Some(team)) => team,
		Ok(None) => {
			messages.error("invalid credentials");

			let mut login_url = "/login".to_string();
			if let Some(next) = creds.next {
				login_url = format!("{}?next={}", login_url, next);
			};

			return Redirect::to(&login_url).into_response();
		}
		Err(_) => return StatusCode::INTERNAL_SERVER_ERROR.into_response(),
	};

	if auth_session.login(&team).await.is_err() {
		return StatusCode::INTERNAL_SERVER_ERROR.into_response();
	}

	messages.success(format!("successfully logged in as {}", team.username));

	if let Some(ref next) = creds.next {
		Redirect::to(next)
	} else {
		Redirect::to("/")
	}
	.into_response()
}
