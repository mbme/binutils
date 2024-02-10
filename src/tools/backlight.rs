use anyhow::{Context, Result};

use crate::{run_command, send_notification};

pub struct Backlight {
    step: f32,
    min: f32,
    max: f32,
    keyboard_backlight: bool,
}

impl Backlight {
    pub fn display_control() -> Self {
        Backlight {
            step: 5.0,
            min: 1.0,
            max: 100.0,
            keyboard_backlight: false,
        }
    }

    pub fn keyboard_control() -> Self {
        Backlight {
            step: 1.0,
            min: 0.0,
            max: 3.0,
            keyboard_backlight: true,
        }
    }

    pub fn inc(&self) {
        let level = self.get_level();

        self.set_level(level + self.step);
    }

    pub fn dec(&self) {
        let level = self.get_level();

        self.set_level(level - self.step);
    }

    fn find_kbd_backlight_device() -> Result<String> {
        let output = run_command("light", vec!["-L"]).context("failed to list devices")?;

        output
            .lines()
            .find_map(|line| {
                if line.contains("kbd_backlight") {
                    Some(line.trim().to_string())
                } else {
                    None
                }
            })
            .context("Failed to find keyboard backlight devices")
    }

    fn run_command(&self, args: Vec<&str>) -> Result<String> {
        let mut args: Vec<String> = args.into_iter().map(|arg| arg.to_string()).collect();

        if self.keyboard_backlight {
            let device = Backlight::find_kbd_backlight_device()?;
            args.append(&mut vec!["-s".to_string(), device, "-r".to_string()]);
        }

        let args: Vec<&str> = args.iter().map(AsRef::as_ref).collect();
        run_command("light", args).context("must be able to set backlight level")
    }

    fn set_level(&self, level: f32) {
        let level: f32 = level.max(self.min).min(self.max);

        self.run_command(vec!["-S", &level.to_string()])
            .expect("must be able to set backlight level");
    }

    fn get_level(&self) -> f32 {
        let output = self
            .run_command(vec!["-G"])
            .expect("must be able to get backlight level");
        let output = output.trim();

        let level: f32 = output
            .parse()
            .expect("must be able to parse backlight level");

        level.round()
    }

    pub fn print_status(&self, notify: bool) {
        let level = self.get_level();

        let message = format!(
            "{} backlight level: {level} [{}-{}]",
            if self.keyboard_backlight {
                "Keyboard"
            } else {
                "Display"
            },
            self.min,
            self.max,
        );

        println!("{}", &message);

        if notify {
            send_notification(&message);
        }
    }
}
