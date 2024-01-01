use std::process::{Command, Stdio};

pub fn cargo<'a>(args: Vec<&'a str>) {
    let mut command = Command::new("cargo");
    command
        .args(args)
        .stdin(Stdio::inherit())
        .stdout(Stdio::inherit())
        .stderr(Stdio::inherit());

    let output = command.output().expect("Failed to execute 'cargo' command");

    if !output.status.success() {
        let error_message = String::from_utf8_lossy(&output.stderr);
        eprintln!("Cargo command failed: {}", error_message);
    }
}
