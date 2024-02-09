use chrono::{DateTime, Duration, Utc};
use std::sync::Arc;
use tokio::sync::RwLock;

#[derive(Clone)]
pub struct TimeStateInner {
	pub initial_start_time: Option<DateTime<Utc>>,
	pub max_duration: Duration,
	pub is_scoring: bool,
	pub state_last_updated: Option<DateTime<Utc>>,
}

#[derive(Clone)]
pub struct TimeState {
	inner: Arc<RwLock<TimeStateInner>>,
}

impl TimeState {
	pub fn new(max_duration: Duration) -> Self {
		Self {
			inner: Arc::new(RwLock::new(TimeStateInner {
				initial_start_time: None,
				max_duration,
				is_scoring: false,
				state_last_updated: None,
			})),
		}
	}

	pub async fn start(&mut self) {
		let mut inner = self.inner.write().await;
		if inner.initial_start_time.is_none() {
			inner.initial_start_time = Some(Utc::now());
		}

		inner.is_scoring = true;
		inner.state_last_updated = inner.initial_start_time;
	}

	pub async fn stop(&mut self) {
		let mut inner = self.inner.write().await;

		if inner.initial_start_time.is_some() {
			inner.is_scoring = false;
			inner.state_last_updated = Some(Utc::now());
		}
	}

	pub async fn reset(&mut self) {
		let mut inner = self.inner.write().await;
		inner.initial_start_time = None;
		inner.is_scoring = false;
		inner.state_last_updated = None;
	}

	pub async fn current(&self) -> TimeStateInner {
		let inner = self.inner.read().await;
		return inner.clone();
	}

	pub async fn time_remaining(&self) -> Duration {
		let inner = self.inner.read().await;

		if let (Some(initial_start_time), Some(state_last_updated)) =
			(inner.initial_start_time, inner.state_last_updated)
		{
			let time_elapsed = if inner.is_scoring {
				let now = Utc::now();
				now - initial_start_time
			} else {
				state_last_updated - initial_start_time
			};
			inner.max_duration - time_elapsed
		} else {
			// comp hasn't started yet
			inner.max_duration
		}
	}
}
