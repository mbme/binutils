use anyhow::{Context, Result};
use serde_json::Value;

use crate::{run_command, send_notification};

pub struct Touchpad;

impl Touchpad {
    pub fn is_enabled() -> Result<bool> {
        let output = run_command("swaymsg", vec!["-t", "get_inputs", "-r"])
            .context("failed to list sway inputs")?;

        let output: Vec<Value> =
            serde_json::from_str(&output).context("failed to parse swaymsg output")?;

        let touchpad = output
            .into_iter()
            .find(|value| {
                value
                    .pointer("/type")
                    .map(|typename| typename == "touchpad")
                    .unwrap_or(false)
            })
            .context("coudn't find any touchpad")?;

        let send_events = touchpad
            .pointer("/libinput/send_events")
            .context("coudn't send_events prop")?;

        Ok(send_events == "enabled")
    }

    pub fn enable(enable: bool) {
        run_command(
            "swaymsg",
            vec![
                "input",
                "type:touchpad",
                "events",
                if enable { "enabled" } else { "disabled" },
            ],
        )
        .expect("failed to toggle touchpad state");
    }

    pub fn disable() {
        Touchpad::enable(false);
    }

    pub fn toggle() {
        run_command(
            "swaymsg",
            vec![
                "input",
                "type:touchpad",
                "events",
                "toggle",
                "enabled",
                "disabled",
            ],
        )
        .expect("failed to toggle touchpad state");
    }

    pub fn print_status(notify: bool) {
        let message = format!(
            "Touchpad is {}",
            if Touchpad::is_enabled().expect("failed to read touchpad status") {
                "enabled"
            } else {
                "disabled"
            }
        );

        println!("{}", &message);

        if notify {
            send_notification(&message);
        }
    }
}
