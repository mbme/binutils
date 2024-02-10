use anyhow::{anyhow, Context, Result};
use lazy_static::*;
use regex::Regex;

use crate::{match_str, run_command, send_notification};

pub struct Speakers {
    id: String,
}

struct SpeakersStatus {
    pub muted: bool,
    pub volume: usize,
}

impl Speakers {
    pub fn up(&self) {
        run_command("pactl", vec!["set-sink-volume", &self.id, "+5%"])
            .expect("must be able to increase volume");
    }

    pub fn down(&self) {
        run_command("pactl", vec!["set-sink-volume", &self.id, "-5%"])
            .expect("must be able to decrease volume");
    }

    pub fn mute(&self) {
        run_command("pactl", vec!["set-sink-mute", &self.id, "1"])
            .expect("must be able to mute volume");
    }

    pub fn unmute(&self) {
        run_command("pactl", vec!["set-sink-mute", &self.id, "0"])
            .expect("must be able to unmute volume");
    }

    pub fn toggle(&self) {
        run_command("pactl", vec!["set-sink-mute", &self.id, "toggle"])
            .expect("must be able to toggle mute");
    }

    fn get_id() -> Result<String> {
        lazy_static! {
            static ref ID_RE: Regex = Regex::new(r"Default Sink: (.*)").unwrap();
        }

        let output = run_command("pactl", vec!["info"])?;

        let id = output
            .lines()
            .find_map(|line| match_str(&ID_RE, line))
            .expect("info must have Default Sink line");

        Ok(id)
    }

    #[must_use]
    pub fn find() -> Speakers {
        let id = Speakers::get_id().expect("Speakers must be available");

        Speakers { id }
    }

    fn get_status(&self) -> Result<SpeakersStatus> {
        let output =
            run_command("pactl", vec!["list", "sinks"]).expect("must be able to list sinks");

        let block = output
            .split("\n\n")
            .find(|block| {
                block
                    .lines()
                    .any(|line| line.contains(&format!("Name: {}", &self.id)))
            })
            .context("Failed to find block for device")?;

        let muted = block.lines().any(|line| line.contains("Mute: yes"));

        let volume = block
            .lines()
            .find(|line| line.contains("Volume:"))
            .and_then(|line| {
                lazy_static! {
                    static ref VOLUME_RE: Regex = Regex::new(r"(\d{1,2})%").unwrap();
                }

                match_str(&VOLUME_RE, line)
            })
            .and_then(|volume| volume.parse().ok())
            .ok_or_else(|| anyhow!("Failed to parse volume"))?;

        Ok(SpeakersStatus { muted, volume })
    }

    pub fn print_status(&self, notify: bool) {
        let status = self
            .get_status()
            .expect("must be able to get speakers status");

        let message = format!(
            "Speakers are {}\nVolume {}%",
            if status.muted { "muted" } else { "unmuted" },
            status.volume,
        );

        println!("{}", &message);

        if notify {
            send_notification(&message);
        }
    }
}
