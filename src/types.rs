use clap::{Parser, Subcommand};

#[derive(Debug, Parser)]
#[command(name = "cargo-workspace", bin_name = "cargo-workspace")]
#[command(about = "Cargo utils for workspaces", long_about = None)]
pub struct Cli {
    #[command(subcommand)]
    pub cmd: Commands,
}

#[derive(Debug, Subcommand)]
#[clap(author, version, about, long_about = None)]
pub enum Commands {
    Version {
        /// Name of the folder with package
        name: String,

        /// Version update
        #[command(subcommand)]
        cmd: VersionUpdates,
    },

    Publish {
        #[clap(short, long)]
        dry_run: bool,

        #[clap(short, long)]
        packages: Vec<String>,
    },
}

#[derive(Debug, Subcommand, Clone)]

pub enum VersionUpdates {
    Major,
    Minor,
    Patch,
}
