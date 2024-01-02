use colored::Colorize;
use semver::Version;
use toml_edit::{value, Document};

use crate::term::git;
use crate::{publish, types};

use types::VersionUpdates;

pub struct Updater {
    patched: Vec<String>,
}

impl Updater {
    pub fn new() -> Self {
        Self { patched: vec![] }
    }

    pub fn version(&mut self, folder_name: String, type_of_update: VersionUpdates, publish: bool) {
        let mut updated = vec![folder_name.clone()];

        self.update_version(folder_name, type_of_update);

        for package in &self.patched {
            updated.push(package.clone());
        }

        if publish {
            git::commit("chore: intercrate version updates");
            publish::publish(Some(updated), false, false);
        }
    }

    fn update_version(&mut self, folder_name: String, type_of_update: VersionUpdates) {
        let workspace_toml = std::fs::read_to_string("./Cargo.toml")
            .expect("Could not read workspace Cargo.toml")
            .parse::<Document>()
            .expect("Invalid workspace cargo.toml");

        let mut folder_toml = std::fs::read_to_string(format!("./{folder_name}/Cargo.toml"))
            .expect(&format!("Could not read {folder_name} Cargo.toml"))
            .parse::<Document>()
            .expect(&format!("Invalid {folder_name} cargo.toml"));

        let version = Version::parse(
            &folder_toml["package"]["version"]
                .as_str()
                .expect(&format!("Invalid version in {folder_name} Cargo.toml")),
        )
        .expect(&format!("Invalid version in {folder_name} Cargo.toml"));

        let main_package_name = &folder_toml["package"]["name"].as_str().unwrap().to_string();

        let new_version = match type_of_update {
            VersionUpdates::Major => Version::new(version.major + 1, 0, 0),
            VersionUpdates::Minor => Version::new(version.major, version.minor + 1, 0),
            VersionUpdates::Patch => Version::new(version.major, version.minor, version.patch + 1),
            VersionUpdates::None => version.clone(),
        };

        folder_toml["package"]["version"] = value(new_version.to_string());

        std::fs::write(
            format!("./{folder_name}/Cargo.toml"),
            folder_toml.to_string(),
        )
        .expect("Failed to write to Cargo.toml");

        match type_of_update {
            VersionUpdates::None => {
                println!(
                    "{}",
                    format!("Version not changed for `{main_package_name}`")
                        .green()
                        .bold()
                );
            }
            _ => {
                println!(
                    "{}",
                    format!("Updated {main_package_name} (v{version} -> v{new_version})")
                        .green()
                        .bold()
                );
            }
        }

        let mut to_cascade_update: Vec<String> = vec![];

        for package in workspace_toml["workspace"]["members"].as_array().unwrap() {
            let package_folder = package.as_str().unwrap().to_string();

            let mut cargo_toml = std::fs::read_to_string(format!("./{package_folder}/Cargo.toml"))
                .expect(&format!("Could not read {package_folder} Cargo.toml"))
                .parse::<Document>()
                .expect(&format!("Invalid {package_folder} cargo.toml"));

            let package_name = cargo_toml["package"]["name"].as_str().unwrap();

            let mut changed = false;

            let info_str = format!("Updated version of {main_package_name} in {package_name}");

            match cargo_toml.get("dependencies") {
                Some(v) => match v.get(main_package_name) {
                    Some(_) => {
                        let init_v = &cargo_toml["dependencies"][main_package_name]["version"]
                            .as_str()
                            .unwrap()
                            .to_string();

                        if init_v == &new_version.to_string() {
                            continue;
                        }

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
                        let init_v = &cargo_toml["dependencies"][main_package_name]["version"]
                            .as_str()
                            .unwrap()
                            .to_string();

                        if init_v == &new_version.to_string() {
                            continue;
                        }

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
                if !self.patched.contains(&package_folder) {
                    self.patched.push(package_folder.clone());
                    to_cascade_update.push(package_folder.clone());
                }

                std::fs::write(
                    format!("./{package_folder}/Cargo.toml"),
                    cargo_toml.to_string(),
                )
                .expect("Failed to write to Cargo.toml");
            }
        }

        if !self.patched.contains(&folder_name) {
            for package in to_cascade_update {
                println!("\n{}", format!("Checking {package}").black().bold());
                self.update_version(package, VersionUpdates::Patch);
            }
        }
    }
}
