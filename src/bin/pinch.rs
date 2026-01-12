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
    after_help = "Example:\n pinch -i 'URL'\n\nDependencies:\n yt-dlp: https://github.com/yt-dlp/yt-dlp\n deno: https://deno.com/",
)]
// disable_version_flag allows lowercase -v
// disable_help_flag prevents the naming conflict with the manual 'help' field
#[clap(disable_version_flag = true, disable_help_flag = true)]

struct Args {
    /// Input URL (YouTube, Vimeo, etc.)
    #[arg(short = 'i', required = true)]
    input: String,

    /// Print help
    #[arg(short = 'h', long = "help", action = clap::ArgAction::Help)]
    help: Option<bool>,

    /// Print version
    #[arg(short = 'v', long = "version", action = clap::ArgAction::Version)]
    version: Option<bool>,
}

fn main () {
    let args = Args::parse();

    // Get stream URLs
    let url_output = Command::new("yt-dlp")
        .args(["-g", "--default-search", "ytsearch", "--no-playlist", "-e", "-f", "bestaudio",  &args.input])
        .output()
        .expect("Failed to execute yt-dlp to get stream URLs");

    // FIX: Convert the output to an owned String so the Vec<&str> has a valid reference to borrow from
    let url_string = String::from_utf8_lossy(&url_output.stdout);
    let stream_urls: Vec<&str> = url_string
        .trim()
        .lines()
        .collect();

    println!("url {}", url_string);

    if stream_urls.is_empty() {
        eprintln!("Error: Could not retrieve stream URLs.");
        std::process::exit(1);
    }

}
