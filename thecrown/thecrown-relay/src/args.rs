use std::path::PathBuf;

use clap::Parser;

#[derive(Debug, Parser)]
#[command(author, version, about = "TheCrown central relay")]
pub struct Args {
    /// Configuration file.
    #[arg(short, long, default_value = "relay.toml")]
    pub config: PathBuf,
}
