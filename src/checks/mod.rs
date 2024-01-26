mod ssh;

use crate::config::Config;

trait Check {
    async fn score(&self, config: &Config) -> anyhow::Result<()>;
}
