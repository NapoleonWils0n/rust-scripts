//==============================================================================
// pinch
// Description: pinch send youtube audio to mpd
//==============================================================================

use clap::Parser;
use std::process::Command;

#[derive(Parser, Debug)]
#[command(
    author,
    version,
    about = "pinch send youtube audio to mpd",
    after_help = "Example:\n pinch -i 'URL'\n\nDependencies:\n mpd, mpc:\n yt-dlp: https://github.com/yt-dlp/yt-dlp\n deno: https://deno.com/",
)]
#[clap(disable_version_flag = true, disable_help_flag = true)]
struct Args {
    /// Input URL (YouTube, etc.)
    #[arg(short = 'i', required = true)]
    input: String,

    /// Print help
    #[arg(short = 'h', long = "help", action = clap::ArgAction::Help)]
    help: Option<bool>,

    /// Print version
    #[arg(short = 'v', long = "version", action = clap::ArgAction::Version)]
    version: Option<bool>,
}

fn main() {
    let args = Args::parse();

    // 1. Resolve MPD_HOST with multiple fallbacks
    let mpd_host = std::env::var("MPD_HOST").unwrap_or_else(|_| {
        // Fallback 1: Check XDG_RUNTIME_DIR (NixOS default)
        if let Ok(runtime_dir) = std::env::var("XDG_RUNTIME_DIR") {
            let nix_path = format!("{}/mpd/socket", runtime_dir);
            if std::path::Path::new(&nix_path).exists() {
                return nix_path;
            }
        }

        // Fallback 2: Check ~/.config/mpd/socket (Debian/Manual default)
        let home = std::env::var("HOME")
            .expect("Neither MPD_HOST, XDG_RUNTIME_DIR, nor HOME is set");
        format!("{}/.config/mpd/socket", home)
    });

    // 2. Get the stream URL using yt-dlp
    let url_output = Command::new("yt-dlp")
        .args(["--no-check-certificate", "--no-playlist", "-f", "bestaudio", "-g", &args.input])
        .output()
        .expect("Failed to execute yt-dlp");

    if !url_output.status.success() {
        eprintln!("Error: yt-dlp failed to fetch the URL.");
        std::process::exit(1);
    }

    let url_string = String::from_utf8_lossy(&url_output.stdout).trim().to_string();

    // Replicate the shell script logic for mpc 
    
    // Check if anything is currently playing
    let mpc_current = Command::new("mpc")
        .env("MPD_HOST", &mpd_host)
        .arg("current")
        .output()
        .expect("Failed to execute mpc current");

    let is_playing = !mpc_current.stdout.is_empty();

    if !is_playing {
        // Nothing is playing. Now check if the playlist is empty 
        let mpc_playlist = Command::new("mpc")
            .env("MPD_HOST", &mpd_host)
            .arg("playlist")
            .output()
            .expect("Failed to execute mpc playlist");

        if mpc_playlist.stdout.is_empty() {
            // Playlist empty: add and play 
            mpc_exec(&["add", &url_string], &mpd_host);
            mpc_exec(&["play"], &mpd_host);
        } else {
            // Playlist not empty: clear, add, and play 
            mpc_exec(&["clear"], &mpd_host);
            mpc_exec(&["add", &url_string], &mpd_host);
            mpc_exec(&["play"], &mpd_host);
        }
    } else {
        // Audio is already playing: just insert/add the new URL 
        mpc_exec(&["add", &url_string], &mpd_host);
    }
}

/// Helper function to run mpc commands and inherit environment (like MPD_HOST) 
fn mpc_exec(args: &[&str], host: &str) {
    let status = Command::new("mpc")
        .env("MPD_HOST", host)
        .args(args)
        .status()
        .expect("Failed to execute mpc command");

    if !status.success() {
        eprintln!("mpc command failed: {:?}", args);
    }
}
