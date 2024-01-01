use clap::{Parser, Subcommand};

#[derive(Debug, Parser)]
#[command(name = "cargo-works", bin_name = "cargo-works")]
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

        #[clap(short, long, value_parser, num_args = 1.., value_delimiter = ' ')]
        packages: Vec<String>,
    },
}

#[derive(Debug, Subcommand, Clone)]

pub enum VersionUpdates {
    Major,
    Minor,
    Patch,
    None,
}

#[derive(Debug, Parser)]
#[clap(name = "cargo-works", bin_name = "cargo", version)]
pub enum Cargo {
    Works(Cli),
}
