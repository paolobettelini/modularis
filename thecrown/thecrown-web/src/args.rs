use std::path::PathBuf;

use clap::Parser;

#[derive(Debug, Parser)]
#[command(author, version, about = "TheCrown Actix + Leptos web service")]
pub struct Args {
    /// Configuration file.
    #[arg(short, long, default_value = "web.toml")]
    pub config: PathBuf,
}
