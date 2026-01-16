//==============================================================================
// relay-mpv
// Description: relay a stream to a named pipe with mpv
//==============================================================================

use clap::Parser;
use std::process::Command;
use std::fs;
use std::os::unix::fs::FileTypeExt;
use std::path::Path;

#[derive(Parser, Debug)]
#[command(
    author,
    version,
    about = "relay a stream to a named pipe with mpv",
    after_help = "Example:\n relay-mpv -i 'URL' -s 00:00:00 -e 00:00:00\n\nDependencies:\n mpv: https://mpv.io/\n yt-dlp: https://github.com/yt-dlp/yt-dlp\n deno: https://deno.com/",
)]

#[clap(disable_version_flag = true, disable_help_flag = true)]
struct Args {
    /// Input URL (YouTube, etc.)
    #[arg(short = 'i', required = true)]
    input: String,

    /// Start time
    #[arg(short = 's')]
    start: Option<String>,

    /// End time
    #[arg(short = 'e')]
    end: Option<String>,

    /// Print help
    #[arg(short = 'h', long = "help", action = clap::ArgAction::Help)]
    help: Option<bool>,

    /// Print version
    #[arg(short = 'v', long = "version", action = clap::ArgAction::Version)]
    version: Option<bool>,
}

// main
fn main() {
    let args = Args::parse();

    // 1. Check if the /tmp/relay named pipe exists, otherwise create it 
    let pipe_path = "/tmp/relay";
    if !Path::new(pipe_path).exists() {
        Command::new("mkfifo")
            .arg(pipe_path)
            .status()
            .expect("Failed to create named pipe /tmp/relay");
    } else {
        // Verify it is actually a named pipe
        let metadata = fs::metadata(pipe_path).expect("Failed to get pipe metadata");
        if !metadata.file_type().is_fifo() {
            eprintln!("Error: /tmp/relay exists but is not a named pipe.");
            std::process::exit(1);
        }
    }

    // 2. Prepare mpv arguments 
    let mut mpv_args = vec![
        "--config-dir=/dev/null",
        "--o=/tmp/relay",
        "--of=nut",
        "--ovc=rawvideo",
        "--oac=pcm_s16le",
    ];

    // Add optional start time if provided 
    if let Some(ref s) = args.start {
        mpv_args.push("--start");
        mpv_args.push(s);
    }

    // Add optional end time if provided 
    if let Some(ref e) = args.end {
        mpv_args.push("--end");
        mpv_args.push(e);
    }

    // Add the input URL 
    mpv_args.push(&args.input);

    // 3. Execute mpv 
    let status = Command::new("mpv")
        .args(&mpv_args)
        .status()
        .expect("Failed to execute mpv");

    if !status.success() {
        eprintln!("mpv execution failed");
        std::process::exit(1);
    }

}
