use crate::web::{WebResult, WebState};
use askama_axum::Template;
use axum::extract::State;
use axum::response::IntoResponse;

#[derive(Template)]
#[template(path = "login.html")]
struct Login {
	mock_title: String,
}

pub async fn login(State(ctxt): State<WebState>) -> WebResult<impl IntoResponse> {
	Ok(Login {
		mock_title: ctxt.title(),
	})
}
