use crate::{checks::Check, db::Db, shuffle::ShuffleIterExt};
use std::{net::Ipv4Addr, str::FromStr};

impl crate::Config {
	async fn score(&self, db: &Db) -> anyhow::Result<()> {
		for (team, subnet) in self.teams.iter().shuffle() {
			for vm in self.vms.values().shuffle() {
				for (alias, service) in vm.services.iter().shuffle() {
					// pre-validated
					let ip = Ipv4Addr::from_str(&subnet.replace('x', &vm.ip.to_string())).unwrap();
					let res = service.score(ip, &vm).await;

					db.upsert_service(&team, &alias, res);
				}
			}
		}

		Ok(())
	}
}
