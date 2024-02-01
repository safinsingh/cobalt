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
	let count: i64 = sqlx::query_scalar(
		r#"
			WITH LatestUpTrue AS (
				SELECT MAX(time) as max_time
				FROM service_checks
				WHERE team = $1 AND vm = $2 AND service = $3 AND up = true
			)
			SELECT COUNT(*)
			FROM service_checks
			WHERE
				team = $1 AND vm = $2 AND service = $3 AND 
				time > (SELECT COALESCE(max_time, '1970-01-01') FROM LatestUpTrue)
	"#,
	)
	.bind(team)
	.bind(vm)
	.bind(service)
	.fetch_one(conn)
	.await?;

	Ok(count > 0 && count % (limit as i64) == 0)
}

pub async fn report_sla_violation(
	conn: impl PgExecutor<'_>,
	team: &str,
	vm: &str,
	service: &str,
	time: DateTime<Utc>,
) -> anyhow::Result<()> {
	sqlx::query!(
		r#"
			INSERT INTO sla_violations(team, vm, service, time)
			VALUES ($1, $2, $3, $4);
		"#,
		team,
		vm,
		service,
		time
	)
	.execute(conn)
	.await?;

	Ok(())
}

pub async fn record_service(
	conn: impl PgExecutor<'_>,
	team: &str,
	vm: &str,
	service: &str,
	time: DateTime<Utc>,
	status: &CheckResult,
) -> anyhow::Result<()> {
	let (short, long) = get_check_result_errors(&status);

	sqlx::query!(
		r#"
			INSERT INTO service_checks(team, vm, service, up, short_error, long_error, time)
			VALUES ($1, $2, $3, $4, $5, $6, $7);
		"#,
		team,
		vm,
		service,
		status.is_ok(),
		short,
		long,
		time
	)
	.execute(conn)
	.await?;

	Ok(())
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
