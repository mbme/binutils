use clap::{Parser, Subcommand};

use binutils::{
    get_crate_version,
    tools::{Microphone, Speakers},
};

/// Control speakers and microphone
#[derive(Parser, Debug)]
#[clap(version = get_crate_version(), about, long_about = None, arg_required_else_help = true)]
struct Args {
    /// Show notification with the current speakers and microphone state
    #[clap(short, action, global = true)]
    notify: bool,

    #[clap(subcommand)]
    device: Device,
}

#[derive(Subcommand, Debug)]
enum Device {
    /// Control speakers
    #[clap(subcommand)]
    Speakers(SpeakersCommand),

    /// Control microphone
    #[clap(name = "mic", subcommand)]
    Microphone(MicrophoneCommand),
}

#[derive(Subcommand, Debug)]
enum SpeakersCommand {
    /// Print current speakers state
    Status,
    /// Mute speakers
    Mute,
    /// Unmute speakers
    Unmute,
    /// Toggle mute
    Toggle,
    /// Increase volume
    Up,
    /// Decrease volume
    Down,
}

#[derive(Subcommand, Debug)]
enum MicrophoneCommand {
    /// Print current microphone state
    Status,
    /// Mute microphone
    Mute,
    /// Unmute microphone
    Unmute,
    /// Toggle mute
    Toggle,
}

fn main() {
    env_logger::init();

    let args = Args::parse();

    match args.device {
        Device::Speakers(command) => {
            let speakers = Speakers::find();
            match command {
                SpeakersCommand::Status => {
                    speakers.print_status(args.notify);
                }
                SpeakersCommand::Mute => {
                    speakers.mute();
                    speakers.print_status(args.notify);
                }
                SpeakersCommand::Unmute => {
                    speakers.unmute();
                    speakers.print_status(args.notify);
                }
                SpeakersCommand::Toggle => {
                    speakers.toggle();
                    speakers.print_status(args.notify);
                }
                SpeakersCommand::Up => {
                    speakers.up();
                    speakers.print_status(args.notify);
                }
                SpeakersCommand::Down => {
                    speakers.down();
                    speakers.print_status(args.notify);
                }
            }
        }
        Device::Microphone(command) => {
            let mic = Microphone::find();
            match command {
                MicrophoneCommand::Status => {
                    mic.print_status(args.notify);
                }
                MicrophoneCommand::Mute => {
                    mic.mute();
                    mic.print_status(args.notify);
                }
                MicrophoneCommand::Unmute => {
                    mic.unmute();
                    mic.print_status(args.notify);
                }
                MicrophoneCommand::Toggle => {
                    mic.toggle();
                    mic.print_status(args.notify);
                }
            }
        }
    }
}
