use crate::{
	checks::{errors::get_check_result_errors, CheckResult},
	config::Scoring,
	db::models::ServiceMap,
};
use chrono::{DateTime, Utc};
use sqlx::{types::Json, PgExecutor};

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
	let (short, long) = get_check_result_errors(status);

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
