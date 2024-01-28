use crate::checks::CheckResult;
use std::borrow::Cow;

impl super::Db {
	pub async fn upsert_service(
		&self,
		team: &str,
		service: &str,
		status: CheckResult,
	) -> anyhow::Result<()> {
		let short_err: Cow<str> = status
			.as_ref()
			.map_or_else(|e| e.short.to_string().into(), |_| "".into());
		let verbose_err: Cow<str> = status
			.as_ref()
			.map_or_else(|e| e.verbose.to_string().into(), |_| "".into());

		sqlx::query!(
			r#"
WITH team_id_subquery AS (
	SELECT id FROM teams WHERE alias = $1
), last_status_subquery AS (
	SELECT up, consecutive_downs FROM services
	WHERE team_id = (SELECT id FROM team_id_subquery) AND alias = $2
)
INSERT INTO services (team_id, alias, consecutive_downs, up, short_error, verbose_error)
SELECT
	(SELECT id FROM team_id_subquery),
	$2,
	COALESCE(
		CASE
			WHEN $3 = FALSE THEN (SELECT consecutive_downs FROM last_status_subquery) + 1
			ELSE 0
		END,
		0 -- Handle service not being inserted yet
	),
	$3,
	$4,
	$5
FROM team_id_subquery
ON CONFLICT (team_id, alias) DO UPDATE
SET
	consecutive_downs = EXCLUDED.consecutive_downs,
	up = EXCLUDED.up,
	short_error = EXCLUDED.short_error,
	verbose_error = EXCLUDED.verbose_error;
"#,
			team,
			service,
			status.is_ok(),
			&short_err,
			&verbose_err
		)
		.execute(&self.conn)
		.await
		.map(|_| ())?;

		Ok(())
	}
}
