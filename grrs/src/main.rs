/**
 * The main entry point for the grrs application.
 * This file contains the implementation of the `main` function.
 * The `main` function is used to parse command-line arguments, open a file, and call the `find_match` function.
 * 
 */

use anyhow::{Context, Result};
use clap::Parser;
use std::fs::File;
use std::io::{self};
use std::path::PathBuf;

#[derive(Parser)]
struct Cli {
    pattern: String,
    path: PathBuf,
}

fn main() -> Result<()> {
    let args = Cli::parse();

    let file = File::open(&args.path)
        .with_context(|| format!("Could not open file `{}`", args.path.display()))?;
    let reader = io::BufReader::new(file);

    grrs::find_match(reader, &args.pattern, &args.path)?;

    Ok(())
}