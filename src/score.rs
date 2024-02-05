use crate::{
	checks::{errors::check_error, Check},
	config::Config,
	db::{self, models::ServiceGatheredInfo},
	shuffle::ShuffleIterExt,
};
use chrono::Utc;
use log::{debug, info};
use sqlx::PgPool;
use std::{collections::HashMap, net::Ipv4Addr, str::FromStr, time::Duration};
use tokio::time::timeout;

// 30 second check timeout
const TIMEOUT: u64 = 30;

pub async fn run(cfg: &Config, pool: PgPool) -> anyhow::Result<()> {
	for (team, subnet) in cfg.teams.iter().shuffle() {
		let mut team_snapshot = HashMap::new();
		for (vm_alias, vm) in cfg.vms.iter().shuffle() {
			let mut vm_snapshot = HashMap::new();
			for (service_alias, service) in vm.services.iter().shuffle() {
				debug!(
					"Commencing scoring check for team='{}', vm='{}', service='{}'",
					team, vm_alias, service_alias
				);

				// pre-validated
				let ip = Ipv4Addr::from_str(&subnet.replace('x', &vm.ip.to_string())).unwrap();
				let time = Utc::now();
				let res = timeout(
					Duration::from_secs(cfg.timing.timeout as u64),
					service.score(ip, &vm),
				)
				.await
				.map_err(|_| {
					check_error!(
						"Timed out",
						format!("Timed out after {} seconds", cfg.timing.timeout)
					)
				})
				.and_then(|ok| ok);

				info!(
					"Result of scoring check for team='{}', vm='{}', service='{}': {:?}",
					team, vm_alias, service_alias, res
				);

				db::mutation::record_service(&pool, &team, &vm_alias, &service_alias, time, &res)
					.await?;

				let incurred_sla = db::query::check_sla_violation(
					&pool,
					&team,
					&vm_alias,
					&service_alias,
					cfg.slas.max_consecutive_downs,
				)
				.await?;
				if incurred_sla {
					info!(
						"SLA incurred for team='{}', vm='{}', service='{}'",
						team, vm_alias, service_alias
					);
					db::mutation::report_sla_violation(
						&pool,
						&team,
						&vm_alias,
						&service_alias,
						time,
					)
					.await?;
				}

				vm_snapshot.insert(
					service_alias.as_str(),
					ServiceGatheredInfo {
						up: res.is_ok(),
						incurred_sla,
					},
				);
			}
			team_snapshot.insert(vm_alias.as_str(), vm_snapshot);
		}

		let time = Utc::now();
		db::mutation::snapshot_team(&pool, &team, team_snapshot, cfg.scoring, time).await?;
	}

	Ok(())
}
