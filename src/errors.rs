#[derive(Debug)]
pub struct CheckError {
	pub short: anyhow::Error,
	pub verbose: anyhow::Error,
}

impl CheckError {
	pub fn new(short: anyhow::Error, verbose: anyhow::Error) -> Self {
		Self { short, verbose }
	}
}

impl std::fmt::Display for CheckError {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		write!(f, "{}", self.short)
	}
}

impl std::error::Error for CheckError {}

macro_rules! check_bail {
	($short:expr, $verbose:expr) => {
		return Err(crate::errors::CheckError::new(
			anyhow::anyhow!($short),
			anyhow::anyhow!($verbose),
		)
		.into())
	};
}
pub(crate) use check_bail;
