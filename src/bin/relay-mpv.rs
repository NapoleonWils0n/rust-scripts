//==============================================================================
// relay-mpv
// Description: relay a stream to a named pipe with mpv
//==============================================================================

use clap::Parser;
use std::process::Command;
use std::fs::{self, OpenOptions};
use std::io::Read;
use std::os::unix::fs::{FileTypeExt, OpenOptionsExt};
use std::path::Path;

#[derive(Parser, Debug)]
#[command(
    author,
    version,
    about = "relay a stream to a named pipe with mpv",
    after_help = "Example:\n relay-mpv -i 'URL' -s 00:00:00 -e 00:00:10\n\nDependencies:\n mpv, yt-dlp",
)]
#[clap(disable_version_flag = true, disable_help_flag = true)]
struct Args {
    /// Input URL (YouTube, etc.)
    #[arg(short = 'i', required = true)]
    input: String,

    /// Start time (optional)
    #[arg(short = 's')]
    start: Option<String>,

    /// End time (optional)
    #[arg(short = 'e')]
    end: Option<String>,

    /// Print help
    #[arg(short = 'h', long = "help", action = clap::ArgAction::Help)]
    help: Option<bool>,

    /// Print version
    #[arg(short = 'v', long = "version", action = clap::ArgAction::Version)]
    version: Option<bool>,
}

fn main() {
    let args = Args::parse();
    let pipe_path = "/tmp/relay";

    // 1. Check if the /tmp/relay named pipe exists, otherwise create it
    if !Path::new(pipe_path).exists() {
        Command::new("mkfifo")
            .arg(pipe_path)
            .status()
            .expect("Failed to create named pipe /tmp/relay");
    } else {
        let metadata = fs::metadata(pipe_path).expect("Failed to get pipe metadata");
        if !metadata.file_type().is_fifo() {
            eprintln!("Error: /tmp/relay exists but is not a named pipe.");
            std::process::exit(1);
        }
    }

    // 2. PRE-START FLUSH: Empty the pipe before mpv starts
    // This prevents "Writing packet failed" by clearing stale data
    if let Ok(mut pipe) = OpenOptions::new()
        .read(true)
        .custom_flags(libc::O_NONBLOCK)
        .open(pipe_path) 
    {
        let mut buffer = [0; 8192];
        while let Ok(n) = pipe.read(&mut buffer) {
            if n == 0 { break; } 
        }
    }

    // 3. Prepare mpv arguments 
    let mut mpv_args = vec![
        "--o=/tmp/relay",
        "--of=nut",
        "--ovc=rawvideo",
        "--oac=pcm_s16le",
        "--msg-level=all=status,ffmpeg=fatal",
        "--ytdl-raw-options=sleep-interval=0,max-sleep-interval=0,socket-timeout=5,no-playlist=",
    ];

    let start_arg;
    if let Some(ref s) = args.start {
        start_arg = format!("--start={}", s);
        mpv_args.push(&start_arg);
    }

    let end_arg;
    if let Some(ref e) = args.end {
        end_arg = format!("--end={}", e);
        mpv_args.push(&end_arg);
    }

    mpv_args.push(&args.input);

    // 4. Execute mpv 
    let status = Command::new("mpv")
        .args(&mpv_args)
        .status()
        .expect("Failed to execute mpv");

    // 5. POST-EXIT FLUSH: Cleanup for next run
    if let Ok(mut pipe) = OpenOptions::new()
        .read(true)
        .custom_flags(libc::O_NONBLOCK)
        .open(pipe_path) 
    {
        let mut buffer = [0; 8192];
        while let Ok(n) = pipe.read(&mut buffer) {
            if n == 0 { break; } 
        }
    }

    if !status.success() {
        eprintln!("mpv execution finished with non-zero status");
        std::process::exit(status.code().unwrap_or(1));
    }
}
