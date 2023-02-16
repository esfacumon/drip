use clap::Parser;
use std::io::{self, Write};

/// Search for a pettern in a file and display the lines that cointain it.
#[derive(Parser)]
struct Cli {
    /// The patterns to look for
    pattern: String,
    /// The path to the file to read
    path: std::path::PathBuf,
}


fn main() {
    let stdout = io::stdout(); // get the global stdout entity
    let mut handle = io::BufWriter::new(stdout);

    let args = Cli::parse();
    let content = std::fs::read_to_string(&args.path).unwrap();


    for line in content.lines() {
        if line.contains(&args.pattern) {
            writeln!(handle, "{}", line);
        }
    }
}