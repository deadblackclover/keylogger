//! Register various user actions - keystrokes on the computer keyboard, movements and mouse keystrokes
//!
//! ## Install
//!
//! If you have Rust: `cargo install keylogger`
//!
//! ## Usage
//!
//! ```bash
//! Register various user actions - keystrokes on the computer keyboard, movements and mouse keystrokes
//! 
//! Usage: keylogger [PATH]
//! 
//! Arguments:
//!   [PATH]  [default: .keylogger]
//!
//! Options:
//!   -h, --help     Print help
//!   -V, --version  Print version
//! ```
mod keylogger;

use clap::Parser;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    #[arg(default_value_t = String::from(".keylogger"))]
    path: String,
}

fn main() {
    let args = Args::parse();
    keylogger::run(args.path);
}
