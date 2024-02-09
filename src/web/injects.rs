use crate::{
	auth::AuthSession,
	config::Inject,
	web::{BaseTemplate, WebState},
};
use askama::Template;
use axum::{extract::State, http::StatusCode, response::IntoResponse};

#[derive(Template)]
#[template(path = "injects.html")]
struct InjectsTemplate<'a> {
	base: BaseTemplate,
	injects: &'a [Inject],
}

pub async fn get(State(ctxt): State<WebState>, auth_session: AuthSession) -> impl IntoResponse {
	if auth_session.user.is_some() {
		let injects = &ctxt.config.injects;

		InjectsTemplate {
			injects,
			base: BaseTemplate::from_params(&ctxt, auth_session).await,
		}
		.into_response()
	} else {
		StatusCode::INTERNAL_SERVER_ERROR.into_response()
	}
}

pub mod page {
	use super::*;
	use axum::extract::Path;

	#[derive(Template)]
	#[template(path = "inject_page.html")]
	struct InjectPageTemplate<'a> {
		base: BaseTemplate,
		inject: &'a Inject,
	}

	pub async fn get(
		State(ctxt): State<WebState>,
		auth_session: AuthSession,
		Path(inject_number): Path<usize>,
	) -> impl IntoResponse {
		if auth_session.user.is_some() {
			if inject_number >= ctxt.config.injects.len() {
				// todo better error
				StatusCode::INTERNAL_SERVER_ERROR.into_response()
			} else {
				InjectPageTemplate {
					base: BaseTemplate::from_params(&ctxt, auth_session).await,
					inject: &ctxt.config.injects[inject_number],
				}
				.into_response()
			}
		} else {
			StatusCode::INTERNAL_SERVER_ERROR.into_response()
		}
	}
}
