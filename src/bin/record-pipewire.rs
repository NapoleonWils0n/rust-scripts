use clap::Parser;
use std::io::{self, Write};
use std::process::{Command, Stdio};
use std::time::{Duration, Instant};
use std::sync::Arc;
use std::sync::atomic::{AtomicBool, Ordering};
use crossterm::event::{self, Event, KeyCode};
use crossterm::terminal::{disable_raw_mode, enable_raw_mode};

#[derive(Parser, Debug)]
#[command(author, version, about = "Record PipeWire audio in Rust")]
struct Args {
    /// Output file name
    #[arg(short = 'o')]
    output: Option<String>,
}

fn main() -> io::Result<()> {
    let args = Args::parse();
    
    // Default naming convention from your original script [cite: 21]
    let timestamp = chrono::Local::now().format("%Y-%m-%d-%H-%M-%S").to_string();
    let default_name = format!("pipewire-{}.wav", timestamp);
    let filename = args.output.unwrap_or(default_name);

    println!("Recording to: {}", filename);
    println!("Press 'q' to stop recording...");

    // Start pw-record as a child process [cite: 24]
    let mut child = Command::new("pw-record")
        .arg("-P")
        .arg("{ stream.capture.sink=true }")
        .arg(&filename)
        .stdout(Stdio::null())
        .stderr(Stdio::null())
        .spawn()
        .expect("Failed to start pw-record");

    let start_time = Instant::now();
    enable_raw_mode()?; // Enter raw mode to capture 'q' without Enter

    loop {
        // 1. Check if 'q' was pressed
        if event::poll(Duration::from_millis(100))? {
            if let Event::Key(key) = event::read()? {
                if key.code == KeyCode::Char('q') {
                    println!("\nStopping recording...");
                    let _ = child.kill(); // Gracefully kill pw-record
                    break;
                }
            }
        }

        // 2. Check if the process exited unexpectedly
        if let Ok(Some(_)) = child.try_wait() {
            println!("\npw-record exited unexpectedly.");
            break;
        }

        // 3. Show live output in the terminal
        let elapsed = start_time.elapsed();
        print!("\rRecording: {:02}:{:02}:{:02} ", 
               elapsed.as_secs() / 3600, 
               (elapsed.as_secs() % 3600) / 60, 
               elapsed.as_secs() % 60);
        io::stdout().flush()?;
    }

    disable_raw_mode()?;
    println!("Recording saved to {}", filename);
    Ok(())
}
