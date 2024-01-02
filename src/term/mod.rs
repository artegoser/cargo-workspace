use std::process::{self, Command, Stdio};

pub mod cargo;
pub mod git;

pub fn run_term(command_name: &str, args: Vec<&str>) {
    let mut command = Command::new(command_name);

    command
        .args(args)
        .stdin(Stdio::inherit())
        .stdout(Stdio::inherit())
        .stderr(Stdio::inherit());

    let output = command
        .output()
        .expect(&format!("Failed to execute '{command_name}' command"));

    if !output.status.success() {
        let error_message = String::from_utf8_lossy(&output.stderr);
        eprintln!("{command_name} failed: {error_message}");
        process::exit(1);
    }
}

pub fn run_with_subcommand(command: &str, subcommand: &str, args: Vec<&str>) {
    let mut args = args;
    args.insert(0, subcommand);
    run_term(command, args);
}
