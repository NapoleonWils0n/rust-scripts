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
    after_help = "Example:\n pinch -i 'URL'\n\nDependencies:\n yt-dlp: https://github.com/yt-dlp/yt-dlp\n  deno: https://deno.com/",
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
