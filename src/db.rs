struct TeamSnapshot {
	alias: String,
	points: i32,
	time: chrono::Duration,
}

struct Service {
	alias: String,
	consecutive_downs: u32,
}

struct Uptime {
	up: bool,
	error: Option<String>,
}

struct SlaViolation {
	time: chrono::DateTime<chrono::Utc>,
}
