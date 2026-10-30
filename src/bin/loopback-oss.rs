//==============================================================================
// loopback-oss
// Description: Create virtual_oss loopback device on FreeBSD for desktop audio
//==============================================================================

// Import command-line argument parsing traits and macros from the clap crate
use clap::{Parser, CommandFactory};
// Import shared helper functions from library crate
use rust_scripts::{run_command};


/// CLI arguments structure parsed via `clap` to handle user input flags.
#[derive(Parser, Debug)]
#[command(
    author,
    version,
    about = "virtual_oss loopback device on /dev/dsp to record desktop audio",
    after_help = "loopback-oss",
    override_usage = "loopback-oss"
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


/// Verifies that the program is being executed with root (UID 0) privileges.
/// Prints help text and exits with an error if run as a standard user.
fn check_root() {
    if unsafe { libc::geteuid() } != 0 {
        eprintln!("Error: must be run as root.\n");
        let _ = Args::command().print_help();
        println!();
        std::process::exit(1);
    }
}


/// Main entry point: checks root permissions, parses command line options, 
fn main() {
    // 1. Ensure root privileges
    check_root();

    // 2. Parse arguments (handles -h and -v automatically)
    let _args = Args::parse();

    // 3. Execute the virtual_oss loopback command
    let virtual_oss_args = [
        "virtual_oss",
        "-Q", "0",
        "-C", "2",
        "-c", "2",
        "-r", "48000",
        "-b", "16",
        "-s", "1024",
        "-P", "/dev/dsp1",
        "-R", "/dev/null",
        "-w", "vdsp.wav",
        "-l", "dsp",
        "-t", "dsp.ctl",
    ];

    if let Err(e) = run_command(&virtual_oss_args) {
        eprintln!("Error executing virtual_oss: {}", e);
        std::process::exit(1);
    }
}
