#![deny(clippy::all)]
#![warn(clippy::nursery, clippy::pedantic, clippy::cargo)]
#![allow(clippy::multiple_crate_versions)]
mod cmd;
use cmd::parse_args;
mod notify;
use notify::pomodoros_launch;

fn main() {
    let args = match parse_args() {
        Ok(v) => v,
        Err(e) => {
            eprintln!("Error: {}.", e);
            std::process::exit(1);
        }
    };

    println!("Pomodoro timer started! Ctrl-C to abort sending notifications.");
    pomodoros_launch(args);
}
