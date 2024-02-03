use super::{WebResult, WebState};
use crate::db::{self, models::ServiceGatheredInfo, query::LatestTeamSnapshot};
use askama_axum::Template;
use axum::{extract::State, response::IntoResponse};
use chrono::{DateTime, Local};

type FlattenedServices = Vec<(String, Vec<(String, ServiceGatheredInfo)>)>;

#[derive(Template)]
#[template(path = "service_statuses.html")]
struct ServiceStatusTpl {
	// service names altered to be <vm>-<service>
	status_table: FlattenedServices,
	vm_service_names: Vec<String>,
	latest_time: DateTime<Local>,
}

fn flatten_team_snapshots(snapshots: Vec<LatestTeamSnapshot>) -> (FlattenedServices, Vec<String>) {
	let mut teams = Vec::new();
	let mut vm_service_names = Vec::new();
	let mut gathered_service_names = false;

	for snapshot in snapshots.into_iter() {
		let mut vm_services = Vec::new();
		for (vm, services) in snapshot.services.0.into_iter() {
			for (service, info) in services {
				let vm_service = format!("{}-{}", vm, service);
				if !gathered_service_names {
					vm_service_names.push(vm_service.clone());
				}
				vm_services.push((vm_service, info));
			}
			if !gathered_service_names {
				vm_service_names.sort();
				gathered_service_names = true;
			}
		}
		vm_services.sort_by(|(v1, _), (v2, _)| v1.cmp(v2));
		teams.push((snapshot.team, vm_services));
	}

	teams.sort_by(|(t1, _), (t2, _)| t1.cmp(t2));
	(teams, vm_service_names)
}

pub async fn service_statuses(State(ctxt): State<WebState>) -> WebResult<impl IntoResponse> {
	let teams = db::query::latest_service_statuses(&ctxt.pool).await?;
	let latest_time =
		DateTime::<Local>::from(teams.iter().map(|t| t.time).max().unwrap_or_default());
	let (status_table, vm_service_names) = flatten_team_snapshots(teams);

	Ok(ServiceStatusTpl {
		status_table,
		vm_service_names,
		latest_time,
	})
}
