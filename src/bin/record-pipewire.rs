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
struct Args {
    /// Optional prefix (defaults to 'pipewire')
    #[arg(short = 'p', default_value = "pipewire")]
    prefix: String,
}

fn main() -> io::Result<()> {
    let args = Args::parse();
    
    // 1. Temporary filename to use during recording
    let temp_filename = format!("{}-temp.wav", args.prefix);

    println!("Press 'q' to stop recording...");

    // Start pw-record 
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

    let mut final_duration = Duration::new(0, 0);

    loop {
        if event::poll(Duration::from_millis(100))? {
            if let Event::Key(key) = event::read()? {
                if key.code == KeyCode::Char('q') {
                    final_duration = start_time.elapsed();
                    println!("\r\nStopping recording...");
                    let _ = child.kill();
                    let _ = child.wait(); // Ensure process is fully closed
                    break;
                }
            }
        }

        if let Ok(Some(_)) = child.try_wait() {
            final_duration = start_time.elapsed();
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

    // 2. Format the final filename with duration
    let duration_str = format!(
        "{:02}:{:02}:{:02}",
        final_duration.as_secs() / 3600,
        (final_duration.as_secs() % 3600) / 60,
        final_duration.as_secs() % 60
    );
    
    let final_filename = format!("{}-[{}].wav", args.prefix, duration_str);

    // 3. Rename the temporary file to the final duration-based name
    if fs::metadata(&temp_filename).is_ok() {
        fs::rename(&temp_filename, &final_filename)?;
        println!("Recording saved to: {}", final_filename);
    } else {
        println!("Error: Recording file was not created.");
    }

    Ok(())
}
