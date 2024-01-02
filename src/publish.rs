use colored::Colorize;
use toml_edit::Document;

use crate::term::cargo;

pub fn publish(packages: Option<Vec<String>>, dry_run: bool, all: bool) {
    let packages = if all {
        let toml = std::fs::read_to_string("./Cargo.toml")
            .expect("Could not read workspace Cargo.toml")
            .parse::<Document>()
            .expect("Invalid workspace cargo.toml");

        toml["workspace"]["members"]
            .as_array()
            .unwrap()
            .iter()
            .map(|s| s.as_str().unwrap().to_string())
            .collect()
    } else {
        packages.unwrap_or_default()
    };

    for package in packages {
        let cargo_toml = std::fs::read_to_string(format!("./{package}/Cargo.toml"))
            .expect(&format!("Could not read {package} Cargo.toml"))
            .parse::<Document>()
            .expect("Invalid package cargo.toml");

        let package_name = cargo_toml["package"]["name"].as_str().unwrap();

        println!(
            "\n------------------| {} |------------------\n",
            package_name.green().bold()
        );

        cargo::publish(&package_name, dry_run);
    }
}
