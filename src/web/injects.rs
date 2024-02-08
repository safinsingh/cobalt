use crate::{auth::AuthSession, web::WebState};
use askama::Template;
use axum::{extract::State, http::StatusCode, response::IntoResponse};

use crate::web::BaseTemplate;

#[derive(Template)]
#[template(path = "injects.html")]
struct InjectsTemplate {
	base: BaseTemplate,
}

pub async fn get(State(ctxt): State<WebState>, auth_session: AuthSession) -> impl IntoResponse {
	if auth_session.user.is_some() {
		InjectsTemplate {
			base: BaseTemplate::from_params(ctxt.config, auth_session),
		}
		.into_response()
	} else {
		StatusCode::INTERNAL_SERVER_ERROR.into_response()
	}
}
