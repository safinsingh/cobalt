use crate::db::models::ServiceGatheredInfo;
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::{prelude::FromRow, types::Json, PgExecutor};
use std::collections::HashMap;

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

pub type OwnedServiceMap = HashMap<String, HashMap<String, ServiceGatheredInfo>>;

#[derive(Serialize, Deserialize, FromRow)]
pub struct LatestTeamSnapshot {
	pub team: String,
	pub points: i32,
	// { [vm: string]: { [service: string]: boolean } }
	pub services: Json<OwnedServiceMap>,
	pub time: DateTime<Utc>,
}

pub async fn latest_service_statuses(
	conn: impl PgExecutor<'_>,
) -> anyhow::Result<Vec<LatestTeamSnapshot>> {
	let teams = sqlx::query_as::<_, LatestTeamSnapshot>(
		r#"
		SELECT DISTINCT ON (team) team, points, services, time
		FROM team_snapshots
		ORDER BY team, time DESC;
	"#,
	)
	.fetch_all(conn)
	.await?;

	Ok(teams)
}

#[derive(Serialize, Deserialize, FromRow)]
pub struct TeamProgression {
	pub team: String,
	pub points: i32,
	pub time: DateTime<Utc>,
}

pub async fn team_progressions(conn: impl PgExecutor<'_>) -> anyhow::Result<Vec<TeamProgression>> {
	let progression = sqlx::query_as!(
		TeamProgression,
		"SELECT team, points, time FROM team_snapshots"
	)
	.fetch_all(conn)
	.await?;

	Ok(progression)
}
