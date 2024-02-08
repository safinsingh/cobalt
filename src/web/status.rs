use super::{WebResult, WebState};
use crate::db::query::TeamProgression;
use crate::db::{self, models::ServiceGatheredInfo, query::LatestTeamSnapshot};
use askama_axum::Template;
use axum::{extract::State, response::IntoResponse};
use chrono::{DateTime, Local};
use itertools::Itertools;

type FlattenedServices = Vec<(String, Vec<(String, ServiceGatheredInfo)>)>;
struct TeamInfo {
	team: String,
	services_up: usize,
	services_down: usize,
	points: i32,
}

#[derive(Template)]
#[template(path = "status.html")]
struct ServiceStatusTpl {
	mock_title: String,
	// service names altered to be <vm>-<service>
	status_table: FlattenedServices,
	vm_service_names: Vec<String>,
	latest_time: String,
	team_table: Vec<TeamInfo>,
	team_progressions: Vec<TeamProgression>,
}

fn flatten_team_snapshots(snapshots: &Vec<LatestTeamSnapshot>) -> (FlattenedServices, Vec<String>) {
	let mut teams = Vec::new();
	let mut vm_service_names = Vec::new();
	let mut gathered_service_names = false;

	for snapshot in snapshots.into_iter() {
		let mut vm_services = Vec::new();
		for (vm, services) in snapshot.services.0.iter() {
			for (service, info) in services {
				let vm_service = format!("[{}] {}", vm, service);
				if !gathered_service_names {
					vm_service_names.push(vm_service.clone());
				}
				vm_services.push((vm_service, info.to_owned()));
			}
			if !gathered_service_names {
				vm_service_names.sort();
				gathered_service_names = true;
			}
		}
		vm_services.sort_by(|(v1, _), (v2, _)| v1.cmp(v2));
		teams.push((snapshot.team.to_owned(), vm_services));
	}

	teams.sort_by(|(t1, _), (t2, _)| t1.cmp(t2));
	(teams, vm_service_names)
}

fn extract_team_table(snapshots: &Vec<LatestTeamSnapshot>) -> Vec<TeamInfo> {
	snapshots
		.iter()
		.map(|team| {
			let (services_up, services_down) =
				team.services
					.iter()
					.fold((0, 0), |(mut up, mut down), (_, itx)| {
						for (_, svc) in itx {
							if svc.up {
								up += 1;
							} else {
								down += 1;
							}
						}
						(up, down)
					});

			TeamInfo {
				team: team.team.clone(),
				services_up,
				services_down,
				points: team.points,
			}
		})
		.sorted_by_key(|team| team.points)
		.rev()
		.collect()
}

#[axum_macros::debug_handler]
pub async fn status(State(ctxt): State<WebState>) -> WebResult<impl IntoResponse> {
	let teams = db::query::latest_service_statuses(&ctxt.pool).await?;
	let latest_time =
		DateTime::<Local>::from(teams.iter().map(|t| t.time).max().unwrap_or_default())
			.format("%m/%d/%Y %H:%M %p")
			.to_string();
	let (status_table, vm_service_names) = flatten_team_snapshots(&teams);
	let team_table = extract_team_table(&teams);
	let team_progressions = db::query::team_progressions(&ctxt.pool).await?;

	Ok(ServiceStatusTpl {
		mock_title: ctxt.title(),
		status_table,
		vm_service_names,
		latest_time,
		team_table,
		team_progressions,
	})
}
