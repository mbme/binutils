use clap::{Parser, Subcommand};

use binutils::{get_crate_version, tools::Touchpad};

#[derive(Parser, Debug)]
#[clap(version = get_crate_version(), about, long_about = None, arg_required_else_help = true)]
struct Args {
    /// Show notification with the current touchpad state
    #[clap(short, action, global = true)]
    notify: bool,

    #[clap(subcommand)]
    command: Command,
}

#[derive(Subcommand, Debug)]
enum Command {
    /// Print current touchpad state
    Status,
    /// Enable touchpad
    On,
    /// Disable touchpad
    Off,
    /// Toggle touchpad
    Toggle,
}

fn main() {
    env_logger::init();

    let args = Args::parse();

    match args.command {
        Command::Status => {
            Touchpad::print_status(args.notify);
        }
        Command::On => {
            Touchpad::enable(true);
            Touchpad::print_status(args.notify);
        }
        Command::Off => {
            Touchpad::disable();
            Touchpad::print_status(args.notify);
        }
        Command::Toggle => {
            Touchpad::toggle();
            Touchpad::print_status(args.notify);
        }
    }
}
