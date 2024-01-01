use colored::Colorize;
use toml_edit::Document;

use crate::cargo::cargo;

pub fn publish(dry_run: bool, packages: Vec<String>) {
    for package in packages {
        let cargo_toml = std::fs::read_to_string(format!("./{package}/Cargo.toml"))
            .expect(&format!("Could not read {package} Cargo.toml"))
            .parse::<Document>()
            .expect("Invalid package cargo.toml");

        let package_name = cargo_toml["package"]["name"].as_str().unwrap();

        let args = if dry_run {
            vec!["publish", "-p", package_name, "--dry-run"]
        } else {
            vec!["publish", "-p", package_name]
        };

        println!(
            "\n------------------| {} |------------------\n",
            package_name.green().bold()
        );
        cargo(args);
    }
}
