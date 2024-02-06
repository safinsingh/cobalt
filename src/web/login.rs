use axum::extract::State;
use crate::web::{WebState, WebResult};
use askama_axum::Template;
use axum::response::IntoResponse;

#[derive(Template)]
#[template(path = "login.html")]
struct Login;

pub async fn login(State(ctxt): State<WebState>) -> WebResult<impl IntoResponse> {
	Ok(Login)
}
