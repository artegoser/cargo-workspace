use clap::Parser;

mod cargo;
mod publish;
mod types;
mod version;

use types::*;

fn main() {
    let types::Cargo::Works(args) = types::Cargo::parse();

    match args.cmd {
        Commands::Version { name, cmd } => {
            let mut updater = version::Updater::new();
            updater.update_version(name, cmd);
        }
        Commands::Publish { dry_run, packages } => publish::publish(dry_run, packages),
    }
}
