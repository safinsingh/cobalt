use std::{net::Ipv4Addr, str::FromStr};

use crate::{checks::Check, shuffle::ShuffleIterExt};

impl crate::Config {
	async fn score(&self) -> anyhow::Result<()> {
		for (team, subnet) in self.teams.iter().shuffle() {
			for vm in self.vms.values().shuffle() {
				for service in vm.services.values().shuffle() {
					// pre-validated
					let ip = Ipv4Addr::from_str(&subnet.replace('x', &vm.ip.to_string())).unwrap();
					let res = service.score(ip).await;
				}
			}
		}

		Ok(())
	}
}
