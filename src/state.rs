use chrono::{DateTime, Duration, Utc};
use std::sync::Arc;
use tokio::sync::RwLock;

#[derive(Clone, Copy, Debug)]
pub enum EngineState {
	Uninitialized,
	Started {
		time_started: DateTime<Utc>,
		time_remaining_at_start: Duration,
	},
	Stopped {
		time_remaining: Duration,
	},
}

impl Default for EngineState {
	fn default() -> Self {
		Self::Uninitialized
	}
}

#[derive(Clone, Copy, Default)]
pub struct TimerInner {
	pub competition_length: Duration,
	pub engine_state: EngineState,
}

impl TimerInner {
	fn start(&mut self) {
		use EngineState::*;
		match self.engine_state {
			Uninitialized => {
				self.engine_state = Started {
					time_started: Utc::now(),
					time_remaining_at_start: self.competition_length,
				};
			}
			Stopped { time_remaining } => {
				self.engine_state = Started {
					time_started: Utc::now(),
					time_remaining_at_start: time_remaining,
				}
			}
			Started { .. } => (),
		}
	}
}

#[derive(Clone, Default)]
pub struct Timer {
	pub inner: Arc<RwLock<TimerInner>>,
}

impl std::ops::Deref for Timer {
	type Target = Arc<RwLock<TimerInner>>;

	fn deref(&self) -> &Self::Target {
		&self.inner
	}
}
