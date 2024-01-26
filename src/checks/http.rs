use crate::checks::Check;
use crate::config::check_types::Http;
use crate::config::Config;

impl Check for Http {
    async fn score(&self, config: &Config) -> anyhow::Result<()> {
        let res = reqwest::
    }
}
