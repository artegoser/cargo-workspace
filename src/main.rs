use clap::Parser;

mod publish;
mod term;
mod types;
mod version;

use types::*;

fn main() {
    let types::Cargo::Works(args) = types::Cargo::parse();

    match args.cmd {
        Commands::Version { name, cmd, publish } => {
            let mut updater = version::Updater::new();
            updater.version(name, cmd, publish);
        }
        Commands::Publish {
            dry_run,
            packages,
            all,
        } => publish::publish(packages, dry_run, all),
    }
}
