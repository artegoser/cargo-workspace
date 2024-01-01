use colored::Colorize;
use semver::Version;
use toml_edit::{value, Document};

use crate::types;

use types::VersionUpdates;

pub fn update_version(name: String, type_of_update: VersionUpdates) {
    let workspace_toml = std::fs::read_to_string("./Cargo.toml")
        .expect("Could not read workspace Cargo.toml")
        .parse::<Document>()
        .expect("Invalid workspace cargo.toml");

    let mut package_cargo_toml = std::fs::read_to_string(format!("./{name}/Cargo.toml"))
        .expect(&format!("Could not read {name} Cargo.toml"))
        .parse::<Document>()
        .expect("Invalid package cargo.toml");

    let version = Version::parse(
        &package_cargo_toml["package"]["version"]
            .as_str()
            .expect("invalid version"),
    )
    .expect("invalid version");

    let main_package_name = &package_cargo_toml["package"]["name"]
        .as_str()
        .unwrap()
        .to_string();

    let new_version = match type_of_update {
        VersionUpdates::Major => Version::new(version.major + 1, 0, 0),
        VersionUpdates::Minor => Version::new(version.major, version.minor + 1, 0),
        VersionUpdates::Patch => Version::new(version.major, version.minor, version.patch + 1),
    };

    package_cargo_toml["package"]["version"] = value(new_version.to_string());

    std::fs::write(
        format!("./{name}/Cargo.toml"),
        package_cargo_toml.to_string(),
    )
    .expect("Failed to write to Cargo.toml");

    println!(
        "{}",
        format!("Updated {main_package_name}({version}) to {new_version}")
            .green()
            .bold()
    );

    for package in workspace_toml["workspace"]["members"].as_array().unwrap() {
        let package_str = package.as_str().unwrap();
        let mut cargo_toml = std::fs::read_to_string(format!("./{package_str}/Cargo.toml"))
            .expect(&format!("Could not read {package_str} Cargo.toml"))
            .parse::<Document>()
            .expect("Invalid package cargo.toml");

        let mut changed = false;

        match cargo_toml.get("dependencies") {
            Some(v) => match v.get(main_package_name) {
                Some(_) => {
                    cargo_toml["dependencies"][main_package_name]["version"] =
                        value(new_version.to_string());

                    println!(
                            "{}",
                            format!(
                                "Updated {main_package_name}({version}) to {new_version} in dependencies {}",
                                cargo_toml["package"]["name"]
                            )
                            .black()
                        );

                    changed = true;
                }
                None => {}
            },
            None => {}
        }

        match cargo_toml.get("dev-dependencies") {
            Some(v) => match v.get(main_package_name) {
                Some(_) => {
                    cargo_toml["dev-dependencies"][main_package_name]["version"] =
                        value(new_version.to_string());

                    println!(
                            "{}",
                            format!(
                                "Updated {main_package_name}({version}) to {new_version} in dev-dependencies {}",
                                cargo_toml["package"]["name"]
                            )
                            .black()
                        );

                    changed = true;
                }
                None => {}
            },
            None => {}
        }

        if changed {
            std::fs::write(
                format!("./{package_str}/Cargo.toml"),
                cargo_toml.to_string(),
            )
            .expect("Failed to write to Cargo.toml");

            println!(
                "\n{}",
                format!("Starting cascade patch update of {package_str}")
                    .black()
                    .bold()
            );

            update_version(package_str.to_string(), VersionUpdates::Patch)
        }
    }
}
