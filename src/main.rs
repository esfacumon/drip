use clap::Parser;

/// Search for a pettern in a file and display the lines that cointain it.
#[derive(Parser)]
struct Cli {
    /// The patterns to look for
    pattern: String,
    /// The path to the file to read
    path: std::path::PathBuf,
}


fn main() {
    let args = Cli::parse();
    let content = std::fs::read_to_string(&args.path).expect("could not read file");
    
    for line in content.lines() {
        if line.contains(&args.pattern) {
            println!("{}", line);
        }
    }

    dbg!("done!");
}