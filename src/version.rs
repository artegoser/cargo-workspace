use colored::Colorize;
use semver::Version;
use toml_edit::{value, Document};

use crate::types;

use types::VersionUpdates;

pub struct Updater {
    patched: Vec<String>,
}

impl Updater {
    pub fn new() -> Self {
        Self { patched: vec![] }
    }

    pub fn update_version(&mut self, name: String, type_of_update: VersionUpdates) {
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
            VersionUpdates::None => version.clone(),
        };

        package_cargo_toml["package"]["version"] = value(new_version.to_string());

        std::fs::write(
            format!("./{name}/Cargo.toml"),
            package_cargo_toml.to_string(),
        )
        .expect("Failed to write to Cargo.toml");

        match type_of_update {
            VersionUpdates::None => {}
            _ => {
                println!(
                "{}",
                format!("Updated version of `{main_package_name}` from `v{version}` to `v{new_version}`")
                    .green()
                    .bold()
            );
            }
        }

        let mut to_cascade_update: Vec<String> = vec![];

        for package in workspace_toml["workspace"]["members"].as_array().unwrap() {
            let package_string = package.as_str().unwrap().to_string();

            let mut cargo_toml = std::fs::read_to_string(format!("./{package_string}/Cargo.toml"))
                .expect(&format!("Could not read {package_string} Cargo.toml"))
                .parse::<Document>()
                .expect("Invalid package cargo.toml");

            let package_name_str = cargo_toml["package"]["name"].as_str().unwrap();

            let mut changed = false;

            let info_str = match type_of_update {
                VersionUpdates::None => {
                    format!("Updated version of `{main_package_name}` in `{package_name_str}`",)
                }
                _ => format!(
                    "Updated version of `{main_package_name}` from `v{version}` to `v{new_version}` in `{}`",
                    package_name_str
                ),
            };

            match cargo_toml.get("dependencies") {
                Some(v) => match v.get(main_package_name) {
                    Some(_) => {
                        cargo_toml["dependencies"][main_package_name]["version"] =
                            value(new_version.to_string());

                        println!("{}", info_str.black());

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

                        println!("{}", info_str.black());

                        changed = true;
                    }
                    None => {}
                },
                None => {}
            }

            if changed {
                if !self.patched.contains(&package_string) {
                    let version = {
                        let v = Version::parse(
                            &cargo_toml["package"]["version"]
                                .as_str()
                                .expect("invalid version"),
                        )
                        .expect("invalid version");
                        Version::new(v.major, v.minor, v.patch + 1)
                    };

                    cargo_toml["package"]["version"] = value(version.to_string());

                    self.patched.push(package_string.clone());

                    to_cascade_update.push(package_string.clone());
                }

                std::fs::write(
                    format!("./{package_string}/Cargo.toml"),
                    cargo_toml.to_string(),
                )
                .expect("Failed to write to Cargo.toml");
            }
        }

        if !self.patched.contains(&name) {
            for package in to_cascade_update {
                println!("\n{}", format!("Checking {package}").black().bold());
                self.update_version(package, VersionUpdates::None);
            }
        }
    }
}
