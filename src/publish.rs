use toml_edit::Document;

use crate::cargo::cargo;

pub fn publish(dry_run: bool, packages: Vec<String>) {
    for package in packages {
        let cargo_toml = std::fs::read_to_string(format!("./{package}/Cargo.toml"))
            .expect("Could not read package Cargo.toml")
            .parse::<Document>()
            .expect("Invalid package cargo.toml");

        let package_name = cargo_toml["package"]["name"].as_str().unwrap();

        cargo(&["-p", package_name, if dry_run { "--dry-run" } else { "" }]);
    }
}
