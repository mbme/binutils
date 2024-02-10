use clap::{Parser, Subcommand};

use binutils::{get_crate_version, tools::Backlight};

/// Control brightness of the display or keyboard
#[derive(Parser, Debug)]
#[clap(version = get_crate_version(), about, long_about = None, arg_required_else_help = true)]
struct Args {
    /// Show notification with the current brightness level
    #[clap(short, action, global = true)]
    notify: bool,

    /// Control keyboard brightness
    #[clap(long, action, global = true)]
    keyboard: bool,

    #[clap(subcommand)]
    command: Command,
}

#[derive(Subcommand, Debug)]
enum Command {
    /// Print current brightness level
    Status,
    /// Increase backlight brightness
    #[clap(name = "inc")]
    Increase,
    /// Decrease backlight brightness
    #[clap(name = "dec")]
    Decrease,
}

fn main() {
    env_logger::init();

    let args = Args::parse();

    let backlight = if args.keyboard {
        Backlight::keyboard_control()
    } else {
        Backlight::display_control()
    };

    match args.command {
        Command::Increase => {
            backlight.inc();
            backlight.print_status(args.notify);
        }
        Command::Decrease => {
            backlight.dec();
            backlight.print_status(args.notify);
        }
        Command::Status => {
            backlight.print_status(args.notify);
        }
    }
}
