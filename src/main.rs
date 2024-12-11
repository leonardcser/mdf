mod io;
mod tokenizer;

use clap::Parser;
use std::process;

/// A simple CLI for processing files and folders
#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Cli {
    /// List of input files and folders (at least one is required)
    #[arg(required = true)]
    input: Vec<String>,
}

fn main() {
    let args = Cli::parse();

    if let Err(e) = io::process_paths(&args.input) {
        eprintln!("Error: {}", e);
        process::exit(1);
    }
}
