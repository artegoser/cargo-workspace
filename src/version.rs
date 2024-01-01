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

    let mut meta_publish: Vec<String> = workspace_toml["package"]["metadata"]["workspace"]
        ["publish"]
        .as_array()
        .map(|array| {
            array
                .iter()
                .map(|value| value.as_str().unwrap_or_default().to_string())
                .collect()
        })
        .unwrap_or_else(Vec::new);

    let mut package_cargo_toml = std::fs::read_to_string(format!("./{name}/Cargo.toml"))
        .expect("Could not read package Cargo.toml")
        .parse::<Document>()
        .expect("Invalid package cargo.toml");

    let version = Version::parse(
        &package_cargo_toml["package"]["version"]
            .as_str()
            .expect("invalid version"),
    )
    .expect("invalid version");

    let name = &package_cargo_toml["package"]["name"].to_string();

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

    meta_publish.push(name.to_string());

    println!(
        "{}\n",
        format!("Updated {name}({version}) to {new_version}").green()
    );

    for package in workspace_toml["workspace"]["members"].as_array().unwrap() {
        let mut cargo_toml = std::fs::read_to_string(format!("./{package}/Cargo.toml"))
            .expect("Could not read package Cargo.toml")
            .parse::<Document>()
            .expect("Invalid package cargo.toml");

        let dep = &cargo_toml["dependencies"][name];
        let dev_dep = &cargo_toml["dev-dependencies"][name];

        let mut changed = false;

        if dep.is_table() || dep.is_inline_table() {
            cargo_toml["dependencies"][name]["version"] = value(new_version.to_string());

            if !meta_publish.contains(&package.to_string()) {
                meta_publish.push(name.to_string());
            }

            println!(
                "{}",
                format!(
                    "Updated {name}({version}) to {new_version} in dependencies {}",
                    cargo_toml["package"]["name"]
                )
                .green()
            );

            changed = true;
        } else if dev_dep.is_table() || dev_dep.is_inline_table() {
            cargo_toml["dev-dependencies"][name]["version"] = value(new_version.to_string());

            if !meta_publish.contains(&package.to_string()) {
                meta_publish.push(name.to_string());
            }

            println!(
                "{}",
                format!(
                    "Updated {name}({version}) to {new_version} in dev-dependencies {}",
                    cargo_toml["package"]["name"]
                )
                .green()
            );

            changed = true;
        }

        std::fs::write(format!("./{package}/Cargo.toml"), cargo_toml.to_string())
            .expect("Failed to write to Cargo.toml");

        if changed {
            update_version(
                cargo_toml["package"]["name"].as_str().unwrap().to_string(),
                VersionUpdates::Patch,
            )
        }
    }

    // workspace_toml["package"]["metadata"]["workspace"]["publish"] = meta_publish.into();

    // std::fs::write("./Cargo.toml", workspace_toml.to_string())
    //     .expect("Failed to write to workspace Cargo.toml");
}
