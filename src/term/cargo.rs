use crate::term::run_with_subcommand;

pub fn publish(package_folder: &str, dry_run: bool) {
    let args = if dry_run {
        vec!["-p", package_folder, "--dry-run"]
    } else {
        vec!["-p", package_folder]
    };

    run_with_subcommand("cargo", "publish", args);
}
