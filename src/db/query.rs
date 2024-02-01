use crate::checks::errors::get_check_result_errors;
use crate::checks::CheckResult;
use crate::db::models::ServiceMap;
use chrono::{DateTime, Utc};

pub struct UpsertResult {
	pub up: bool,
}

impl super::Db {
	pub async fn record_service(
		&self,
		team: &str,
		vm: &str,
		service: &str,
		time: DateTime<Utc>,
		status: CheckResult,
	) -> anyhow::Result<UpsertResult> {
		let (short, long) = get_check_result_errors(&status);

		let up = sqlx::query_as!(
			UpsertResult,
			r#"
			INSERT INTO service_checks(team, vm, service, up, short_error, long_error, time)
			VALUES ($1, $2, $3, $4, $5, $6, $7)
			RETURNING up;
		"#,
			team,
			vm,
			service,
			status.is_ok(),
			short,
			long,
			time
		)
		.fetch_one(&self.conn)
		.await?;

		Ok(up)
	}

	pub async fn snapshot_team<'s>(
		&'s self,
		team: &str,
		services: ServiceMap<'s>,
		time: DateTime<Utc>,
	) -> anyhow::Result<()> {
		let mut point_differential = services.iter().fold(0, ||)

		Ok(())
	}
}
