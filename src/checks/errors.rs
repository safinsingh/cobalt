#[derive(Debug)]
pub struct CheckError {
	pub short: anyhow::Error,
	pub long: anyhow::Error,
}

impl CheckError {
	pub fn new(short: anyhow::Error, long: anyhow::Error) -> Self {
		Self { short, long }
	}

	pub fn errors(&self) -> (String, String) {
		(self.short.to_string(), self.long.to_string())
	}
}

impl std::fmt::Display for CheckError {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		write!(f, "{}", self.short)
	}
}

impl<T: std::error::Error + Send + Sync + 'static> From<T> for CheckError {
	fn from(value: T) -> Self {
		Self {
			short: anyhow::anyhow!("Internal server error"),
			long: value.into(),
		}
	}
}

macro_rules! check_bail {
	($short:expr, $long:expr) => {
		return Err(crate::checks::errors::CheckError::new(
			::anyhow::anyhow!($short),
			::anyhow::anyhow!($long),
		))
	};
}
pub(crate) use check_bail;

pub type CheckResult = Result<(), CheckError>;

pub fn get_check_result_errors(res: &CheckResult) -> (String, String) {
	match res {
		Ok(_) => (String::new(), String::new()),
		Err(e) => e.errors(),
	}
}
