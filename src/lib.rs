use std::process::Command;

use anyhow::{bail, Context, Result};
use regex::Regex;

pub mod tools;

pub fn get_crate_version() -> &'static str {
    option_env!("BINUTILS_VERSION").unwrap_or("dev-build")
}

pub fn send_notification(message: &str) {
    run_command("notify-send", vec!["-u", "low", message])
        .expect("must be able to send notification");
}

pub fn run_command(command: &str, args: Vec<&str>) -> Result<String> {
    let output = Command::new(command)
        .args(args)
        .output()
        .context("failed to execute command")?;

    if !output.status.success() {
        let err_str = String::from_utf8(output.stderr)?;
        log::error!("command failed:\n{}\n{}", output.status, err_str);
        bail!("Command '{}' executed with failing error code", command);
    }

    let output_str =
        String::from_utf8(output.stdout).context("failed to convert stdout to string")?;

    Ok(output_str)
}

#[must_use]
pub fn match_str(regex: &Regex, s: &str) -> Option<String> {
    regex.captures(s).map(|captures| {
        captures
            .get(1)
            .expect("group 1 must be present")
            .as_str()
            .to_string()
    })
}
