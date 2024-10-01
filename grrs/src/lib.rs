/**
 * This file contains the implementation of the `find_match` function.
 * This function reads the contents of a file line by line and prints any line that contains a given pattern.
 * The function is used in the `main` function in `grrs/src/main.rs`.
 * 
 * The `find_match` function takes three arguments:
 * - `reader`: a `BufRead` instance that reads the contents of a file line by line.
 * - `pattern`: a string slice that represents the pattern to search for in the file.
 * - `path`: a `PathBuf` instance that represents the path to the file being searched.
 * 
 */

use anyhow::{Context, Result};
use std::io::BufRead;
use std::path::PathBuf;

pub fn find_match<R: BufRead>(reader: R, pattern: &str, path: &PathBuf) -> Result<()> {
    // Iterate through each line in the file
    for line in reader.lines() {
        let line = line.with_context(|| format!("Could not read line from file `{}`", path.display()))?;
        if line.contains(pattern) {
            println!("{}", line);
        }
    }
    Ok(())
}