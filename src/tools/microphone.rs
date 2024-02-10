use anyhow::{Context, Result};
use lazy_static::*;
use regex::Regex;

use crate::{match_str, run_command, send_notification};

pub struct Microphone {
    id: String,
}

struct MicrophoneStatus {
    pub muted: bool,
}

impl Microphone {
    pub fn mute(&self) {
        run_command("pactl", vec!["set-source-mute", &self.id, "1"])
            .expect("must be able to mute microphone");
    }

    pub fn unmute(&self) {
        run_command("pactl", vec!["set-source-mute", &self.id, "0"])
            .expect("must be able to unmute microphone");
    }

    pub fn toggle(&self) {
        run_command("pactl", vec!["set-source-mute", &self.id, "toggle"])
            .expect("must be able to toggle mute microphone");
    }

    fn get_id() -> Result<String> {
        lazy_static! {
            static ref ID_RE: Regex = Regex::new(r"Default Source: (.*)").unwrap();
        }

        let output = run_command("pactl", vec!["info"])?;

        let id = output
            .lines()
            .find_map(|line| match_str(&ID_RE, line))
            .expect("info must have Default Source line");

        Ok(id)
    }

    #[must_use]
    pub fn find() -> Microphone {
        let id = Microphone::get_id().expect("Microphone must be available");

        Microphone { id }
    }

    fn get_status(&self) -> Result<MicrophoneStatus> {
        let output =
            run_command("pactl", vec!["list", "sources"]).expect("must be able to list sources");

        let block = output
            .split("\n\n")
            .find(|block| {
                block
                    .lines()
                    .any(|line| line.contains(&format!("Name: {}", &self.id)))
            })
            .context("Failed to find block for device")?;

        let muted = block.lines().any(|line| line.contains("Mute: yes"));

        Ok(MicrophoneStatus { muted })
    }

    pub fn print_status(&self, notify: bool) {
        let status = self
            .get_status()
            .expect("must be able to get microphone status");

        let message = format!(
            "Microphone is {}",
            if status.muted { "muted" } else { "unmuted" }
        );

        println!("{}", &message);

        if notify {
            send_notification(&message);
        }
    }
}
