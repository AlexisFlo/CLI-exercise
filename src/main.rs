use anyhow::{Context, Result};
use std::fs::File;
use std::io::{ BufRead, BufReader};
use clap::Parser;

/// Search for a pattern in a file and display the lines that contain it.
#[derive(Parser)]
struct Cli {
    // The pattern to look for
    pattern: String,
    // The path to the file to read
    path: std::path::PathBuf,
}

// Exercise for the reader: This is not the best implementation: 
// It will read the whole file into memory – however large the file may be. 
// Find a way to optimize it! (One idea might be to use a BufReader instead
// of read_to_string().)
fn main() -> Result<()>{
    let args = Cli::parse();
    let file = File::open(&args.path)
        .with_context(|| format!("could not rea file `{}`", args.path.display()))?;

    let reader = BufReader::new(file);

    for line in reader.lines() {
        let line = line?;
        if line.contains(&args.pattern) {
            println!("{}", line);
        }
    }

    Ok(())
}
