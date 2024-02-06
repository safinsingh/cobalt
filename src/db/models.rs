use chrono::{serde::ts_seconds, DateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use std::collections::HashMap;

#[derive(Serialize, Deserialize, FromRow)]
pub struct ServiceCheck {
	pub team: String,
	pub service: String,
	pub up: bool,
	pub short_error: Option<String>,
	pub long_error: Option<String>,
	#[serde(with = "ts_seconds")]
	pub time: DateTime<Utc>,
}

#[derive(Serialize, Deserialize, Clone, Copy)]
pub struct ServiceGatheredInfo {
	pub up: bool,
	pub incurred_sla: bool,
}

pub type ServiceMap<'s> = HashMap<&'s str, HashMap<&'s str, ServiceGatheredInfo>>;

#[derive(Serialize, Deserialize, FromRow)]
pub struct TeamSnapshot<'s> {
	pub team: String,
	pub points: i32,
	// { [vm: string]: { [service: string]: boolean } }
	#[serde(borrow)]
	pub services: ServiceMap<'s>,
	#[serde(with = "ts_seconds")]
	pub time: DateTime<Utc>,
}

#[derive(Serialize, Deserialize, FromRow)]
pub struct SlaViolation {
	pub team: String,
	pub service: String,
	#[serde(with = "ts_seconds")]
	pub time: DateTime<Utc>,
}
