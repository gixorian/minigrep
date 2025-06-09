use clap::Parser;
use std::process;

fn main() {
    let args = minigrep::Cli::parse();

    if let Err(e) = minigrep::run(args) {
        eprintln!("Application error: {e}");
        process::exit(1);
    }
}
