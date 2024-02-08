use crate::config::Config;
use async_trait::async_trait;
use axum_login::{AuthUser, AuthnBackend, UserId};
use std::convert::Infallible;

#[derive(Clone)]
pub struct AuthTeam {
	pub username: String,
	password: String,
}

impl AuthUser for AuthTeam {
	type Id = String;

	fn id(&self) -> Self::Id {
		self.username.to_owned()
	}

	fn session_auth_hash(&self) -> &[u8] {
		self.password.as_bytes()
	}
}

impl std::fmt::Debug for AuthTeam {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		f.debug_struct("User")
			.field("username", &self.username)
			.field("password", &"[redacted]")
			.finish()
	}
}

pub struct Credentials {
	pub username: String,
	pub password: String,
	// next page
	pub next: Option<String>,
}

#[async_trait]
impl AuthnBackend for Config {
	type User = AuthTeam;
	type Credentials = Credentials;
	type Error = Infallible;

	async fn authenticate(
		&self,
		creds: Self::Credentials,
	) -> Result<Option<Self::User>, Self::Error> {
		Ok(self
			.teams
			.get(&creds.username)
			.filter(|user| user.password == creds.password)
			.map(|team| AuthTeam {
				username: creds.username,
				password: team.password.to_owned(),
			}))
	}

	async fn get_user(&self, user_id: &UserId<Self>) -> Result<Option<Self::User>, Self::Error> {
		Ok(self.teams.get(&*user_id).map(|team| AuthTeam {
			username: user_id.to_owned(),
			password: team.password.to_owned(),
		}))
	}
}
