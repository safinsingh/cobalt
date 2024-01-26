mod checks;
mod config;
mod offset;

use crate::config::Config;
use std::fs;

fn main() -> anyhow::Result<()> {
    let raw = fs::read_to_string("cobalt.yml")?;
    let cfg = Config::from_str(&raw)?;

    println!("{:?}", cfg);

    Ok(())
}
