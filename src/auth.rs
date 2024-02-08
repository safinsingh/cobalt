use crate::config::Config;
use async_trait::async_trait;
use axum_login::{AuthUser, AuthnBackend, UserId};
use serde::Deserialize;
use std::convert::Infallible;

#[derive(Clone)]
pub struct User {
	pub username: String,
	password: String,
}

impl AuthUser for User {
	type Id = String;

	fn id(&self) -> Self::Id {
		self.username.to_owned()
	}

	fn session_auth_hash(&self) -> &[u8] {
		self.password.as_bytes()
	}
}

impl std::fmt::Debug for User {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		f.debug_struct("User")
			.field("username", &self.username)
			.field("password", &"[redacted]")
			.finish()
	}
}

#[derive(Clone, Deserialize)]
pub struct Credentials {
	pub username: String,
	pub password: String,
	// next page
	pub next: Option<String>,
}

#[async_trait]
impl AuthnBackend for Config {
	type User = User;
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
			.map(|user| User {
				username: creds.username,
				password: user.password.to_owned(),
			}))
	}

	async fn get_user(&self, user_id: &UserId<Self>) -> Result<Option<Self::User>, Self::Error> {
		Ok(self.teams.get(user_id).map(|user| User {
			username: user_id.to_owned(),
			password: user.password.to_owned(),
		}))
	}
}

pub type AuthSession = axum_login::AuthSession<Config>;
