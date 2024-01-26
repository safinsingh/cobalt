use crate::checks::Check;
use crate::config::check_types::Ssh;
use crate::config::Config;

impl Check for Ssh {
    fn score(&self, config: &Config) -> anyhow::Result<()> {
        Ok(())
    }
}
