use chrono::{serde::ts_seconds, DateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use std::collections::HashMap;

#[derive(Serialize, Deserialize, FromRow)]
pub struct Team {
	pub id: i32,
	pub alias: String,
	pub points: i32,
}

#[derive(Serialize, Deserialize, FromRow)]
pub struct Service {
	pub id: i32,
	pub team_id: i32,
	pub alias: String,
	pub consecutive_downs: u32,
	pub up: bool,
	pub error: Option<String>,
}

#[derive(Serialize, Deserialize)]
pub struct ServiceInfo {
	pub service_alias: String,
	pub up: bool,
}

#[derive(Serialize, Deserialize, FromRow)]
pub struct TeamSnapshot {
	pub id: i32,
	pub team_id: i32,
	pub points: i32,
	// { [service_id: int]: { service_alias: string, up: boolean } }
	pub services: HashMap<i32, ServiceInfo>,
	#[serde(with = "ts_seconds")]
	pub time: DateTime<Utc>,
}

#[derive(Serialize, Deserialize, FromRow)]
pub struct SlaViolation {
	pub id: i32,
	pub team_id: i32,
	pub service_id: i32,
	#[serde(with = "ts_seconds")]
	pub time: DateTime<Utc>,
}
