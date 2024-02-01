use crate::checks::errors::get_check_result_errors;
use crate::checks::CheckResult;
use crate::config::Scoring;
use crate::db::models::ServiceMap;
use chrono::{DateTime, Utc};
use sqlx::prelude::FromRow;
use sqlx::types::Json;
use sqlx::PgExecutor;

#[derive(FromRow)]
pub struct ServiceStatus {
	pub up: bool,
}

pub async fn check_sla_violation(
	conn: impl PgExecutor<'_>,
	team: &str,
	vm: &str,
	service: &str,
	limit: u32,
) -> anyhow::Result<bool> {
	let query_string = format!(
		"SELECT * FROM service_checks WHERE team = $1 AND vm = $2 AND service = $3 ORDER BY time DESC LIMIT {}",
		limit
	);

	let records = sqlx::query_as::<_, ServiceStatus>(&query_string)
		.bind(team)
		.bind(vm)
		.bind(service)
		.fetch_all(conn)
		.await?;

	Ok(records.iter().all(|s| !s.up))
}

pub async fn record_service(
	conn: impl PgExecutor<'_>,
	team: &str,
	vm: &str,
	service: &str,
	time: DateTime<Utc>,
	status: &CheckResult,
) -> anyhow::Result<ServiceStatus> {
	let (short, long) = get_check_result_errors(&status);

	let up = sqlx::query_as!(
		ServiceStatus,
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
	.fetch_one(conn)
	.await?;

	Ok(up)
}

pub async fn snapshot_team<'s>(
	conn: impl PgExecutor<'_>,
	team: &str,
	service_map: ServiceMap<'s>,
	scoring_info: Scoring,
	time: DateTime<Utc>,
) -> anyhow::Result<()> {
	let mut point_differential = 0;
	for vm in service_map.values() {
		for service in vm.values() {
			if service.up {
				point_differential += scoring_info.service_up;
			} else {
				point_differential += scoring_info.service_down;
				if service.incurred_sla {
					point_differential += scoring_info.sla;
				}
			}
		}
	}

	sqlx::query!(
		r#"
		INSERT INTO team_snapshots(team, points, services, time)
		VALUES ($1, $2, $3, $4);
	"#,
		team,
		point_differential,
		Json(service_map) as _,
		time
	)
	.execute(conn)
	.await?;

	Ok(())
}
