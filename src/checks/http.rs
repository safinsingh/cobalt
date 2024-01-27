use crate::checks::Check;
use crate::config::check_types::Http;
use crate::config::Vm;
use async_trait::async_trait;
use std::net::Ipv4Addr;
use url::Url;

#[async_trait]
impl Check for Http {
	async fn score(&self, ip: Ipv4Addr, _: &Vm) -> anyhow::Result<()> {
		for page in &self.pages {
			let url_raw = format!("http://{}", ip);
			let client = reqwest::Client::new();
			let mut req = client.request(page.method.to_owned(), Url::parse(&url_raw)?);

			if let Some(headers) = &page.headers {
				for (key, value) in headers {
					req = req.header(key, value);
				}
			}

			if let Some(body) = &page.body {
				req = req.body(body.to_owned());
			}

			let res = req.send().await?.text().await?;
			if let Some(contains) = &page.contains {
				if !res.contains(contains) {
					bail!("") //fixthis
				}
			}
		}

		Ok(())
	}
}
