//==============================================================================
// record-pipewire
// Description: record pipewire audio and rename file with duration on exit
//==============================================================================

use clap::Parser;
use std::io::{self, Write};
use std::process::{Command, Stdio};
use std::time::{Duration, Instant};
use std::fs;
use crossterm::event::{self, Event, KeyCode};
use crossterm::terminal::{disable_raw_mode, enable_raw_mode};

#[derive(Parser, Debug)]
#[command(author, version, about = "Record PipeWire audio with duration in filename")]
#[clap(disable_version_flag = true, disable_help_flag = true)]
struct Args {
    /// Optional prefix
    #[arg(short = 'o', default_value = "pipewire")]
    prefix: String,

    /// Print help
    #[arg(short = 'h', long = "help", action = clap::ArgAction::Help)]
    help: Option<bool>,

    /// Print version
    #[arg(short = 'v', long = "version", action = clap::ArgAction::Version)]
    version: Option<bool>,
}

fn main() -> io::Result<()> {
    let args = Args::parse();
    
    // 1. Temporary filename used during active recording
    let temp_filename = format!("{}-temp.wav", args.prefix);

    println!("Press 'q' to stop recording...");

    // Start pw-record child process
    let mut child = Command::new("pw-record")
        .arg("-P")
        .arg("{ stream.capture.sink=true }")
        .arg(&temp_filename)
        .stdout(Stdio::null())
        .stderr(Stdio::null())
        .spawn()
        .expect("Failed to start pw-record");

    let start_time = Instant::now();
    enable_raw_mode()?;

    loop {
        // Check for 'q' key every 100ms
        if event::poll(Duration::from_millis(100))? {
            if let Event::Key(key) = event::read()? {
                if key.code == KeyCode::Char('q') {
                    println!("\r\nStopping recording...");
                    let _ = child.kill();
                    let _ = child.wait(); 
                    break;
                }
            }
        }

        // Safety: break if the recording process dies on its own
        if let Ok(Some(_)) = child.try_wait() {
            println!("\r\npw-record exited unexpectedly.");
            break;
        }

        let elapsed = start_time.elapsed();
        print!(
            "\rRecording: {:02}:{:02}:{:02} ", 
            elapsed.as_secs() / 3600, 
            (elapsed.as_secs() % 3600) / 60, 
            elapsed.as_secs() % 60
        );
        io::stdout().flush()?;
    }

    disable_raw_mode()?;

    // 2. Capture final duration now that the loop has finished
    let final_duration = start_time.elapsed();

    // 3. Format the final filename
    let duration_str = format!(
        "{:02}:{:02}:{:02}",
        final_duration.as_secs() / 3600,
        (final_duration.as_secs() % 3600) / 60,
        final_duration.as_secs() % 60
    );
    
    let final_filename = format!("{}-[{}].wav", args.prefix, duration_str);

    // 4. Rename the temporary file to include the duration
    if fs::metadata(&temp_filename).is_ok() {
        fs::rename(&temp_filename, &final_filename)?;
        // Adding \r and extra spaces to wipe the previous timer line
        println!("\r\x1B[2KRecording saved to: {}", final_filename);
    } else {
        eprintln!("\rError: Temporary recording file was not found.");
    }

    Ok(())
}
