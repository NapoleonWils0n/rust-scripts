//==============================================================================
// record-desktop-audio
// Description: record desktop audio with sox on Freebsd
//==============================================================================

// Import command-line argument parsing traits and macros from the clap crate
use clap::{Parser, CommandFactory};
// Import shared helper functions from library crate
use rust_scripts::{run_command, run_and_capture};


/// CLI arguments structure parsed via `clap` to handle user input flags.
#[derive(Parser, Debug)]
#[command(
    author,
    version,
    about = "record desktop audio with sox on Freebsd",
    after_help = "record-desktop-audio",
    override_usage = "record-desktop-audio"
)]
#[clap(disable_version_flag = true, disable_help_flag = true)]
struct Args {
    /// Print help
    #[arg(short = 'h', long = "help", action = clap::ArgAction::Help)]
    help: Option<bool>,

    /// Print version
    #[arg(short = 'v', long = "version", action = clap::ArgAction::Version)]
    version: Option<bool>,
}

/// Generates a unique filename using the system's native date command
fn get_timestamp_filename() -> String {
    let timestamp = run_and_capture(&["date", "+%Y-%m-%d-%H-%M-%S"])
        .unwrap_or_else(|_| "fallback".to_string());
    format!("sox-{}.wav", timestamp)
}

/// Main entry point: checks root permissions, parses command line options, 
fn main() {
    let _args = Args::parse();

    let filename = get_timestamp_filename();

    let record_desktop_audio_args = [
        "sox",
        "-r", "48k",
        "-b", "16",
        "-v", "0.99",
        "-t", "oss",
        "/dev/dsp", 
        "-t", "wav", 
        &filename,
    ];

    if let Err(e) = run_command(&record_desktop_audio_args) {
        eprintln!("Error executing record-desktop-audio: {}", e);
        std::process::exit(1);
    }
}
