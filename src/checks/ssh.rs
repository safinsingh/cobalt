use crate::{
	checks::{Check, CheckResult},
	config::{
		check_types::{Ssh, SshAuthType},
		Vm,
	},
};
use async_trait::async_trait;
use std::{io::Read, net::Ipv4Addr};
use tokio::net::TcpStream;

use super::check_bail;

#[async_trait]
impl Check for Ssh {
	async fn score(&self, ip: Ipv4Addr, _: &Vm) -> CheckResult {
		let stream = TcpStream::connect((ip, self.port)).await?;

		let mut sess = ssh2::Session::new()?;
		sess.set_tcp_stream(stream);
		sess.handshake()?;
		match &self.auth {
			SshAuthType::Password { user, password } => sess.userauth_password(user, password),
			SshAuthType::Pubkey {
				user,
				private_key,
				passphrase,
			} => sess.userauth_pubkey_file(user, None, private_key, passphrase.as_deref()),
		}?;

		if let Some(command) = &self.command {
			let mut channel = sess.channel_session()?;
			channel.exec(&command)?;
			let mut res = String::new();
			channel.read_to_string(&mut res)?;
			channel.wait_close()?;

			let exit_code = channel.exit_status()?;
			if exit_code != 0 {
				check_bail!(
					"command failed",
					format!(
						"command '{}' failed with nonzero exit code: '{}' (output: '{}')",
						command, exit_code, res
					)
				);
			}
		}

		Ok(())
	}
}
