use crate::{
	checks::Check,
	db::{models::ServiceGatheredInfo, query},
	shuffle::ShuffleIterExt,
};
use chrono::Utc;
use sqlx::PgPool;
use std::collections::HashMap;
use std::{net::Ipv4Addr, str::FromStr};

impl crate::Config {
	pub async fn score(&self, pool: &PgPool) -> anyhow::Result<()> {
		for (team, subnet) in self.teams.iter().shuffle() {
			let mut team_snapshot = HashMap::new();
			for (vm_alias, vm) in self.vms.iter().shuffle() {
				let mut vm_snapshot = HashMap::new();
				for (service_alias, service) in vm.services.iter().shuffle() {
					// pre-validated
					let ip = Ipv4Addr::from_str(&subnet.replace('x', &vm.ip.to_string())).unwrap();
					let time = Utc::now();
					let res = service.score(ip, &vm).await;

					query::record_service(pool, &team, &vm_alias, &service_alias, time, &res)
						.await?;

					let incurred_sla = query::check_sla_violation(
						pool,
						&team,
						&vm_alias,
						&service_alias,
						self.slas.max_consecutive_downs,
					)
					.await?;
					if incurred_sla {
						query::report_sla_violation(pool, &team, &vm_alias, &service_alias, time)
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
			query::snapshot_team(pool, &team, team_snapshot, self.scoring, time).await?;
		}

		Ok(())
	}
}
