use crate::{checks::Check, db::Db, shuffle::ShuffleIterExt};
use chrono::Utc;
use std::collections::HashMap;
use std::{net::Ipv4Addr, str::FromStr};

impl crate::Config {
	async fn score(&self, db: &Db) -> anyhow::Result<()> {
		for (team, subnet) in self.teams.iter().shuffle() {
			let mut team_snapshot = HashMap::new();
			for (vm_alias, vm) in self.vms.iter().shuffle() {
				let mut vm_snapshot = HashMap::new();
				for (service_alias, service) in vm.services.iter().shuffle() {
					// pre-validated
					let ip = Ipv4Addr::from_str(&subnet.replace('x', &vm.ip.to_string())).unwrap();
					let time = Utc::now();
					let res = service.score(ip, &vm).await;

					let res = db
						.record_service(&team, &vm_alias, &service_alias, time, res)
						.await?;
					vm_snapshot.insert(service_alias.as_str(), res.up);
				}
				team_snapshot.insert(vm_alias.as_str(), vm_snapshot);
			}

			let time = Utc::now();
			db.snapshot_team(&team, team_snapshot, time).await?;
		}

		Ok(())
	}
}
