use clap::Parser;

mod cargo;
mod publish;
mod types;
mod version;

use types::*;

fn main() {
    let args = types::Cli::parse();

    match args.cmd {
        Commands::Version { name, cmd } => version::update_version(name, cmd),
        Commands::Publish { dry_run, packages } => publish::publish(dry_run, packages),
    }
}
