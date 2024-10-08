/**
 * Unit tests for the CLI
 * 
 */

use assert_cmd::prelude::*;
use predicates::prelude::*;
use assert_fs::prelude::*;
use std::process::Command;

#[test]
fn file_doesnt_exist() -> Result<(), Box<dyn std::error::Error>> {
    let mut cmd = Command::cargo_bin("grrs")?;

    cmd.arg("foobar").arg("test/file/doesnt/exist");
    cmd.assert()
        .failure()
        .stderr(predicate::str::contains("Could not open file"));

    Ok(())
}

#[test]
fn find_content_in_file() -> Result<(), Box<dyn std::error::Error>> {
    let file = assert_fs::NamedTempFile::new("sample.txt")?;
    file.write_str("A test\nActual content\nMore content\nAnother test")?;

    let mut cmd = Command::cargo_bin("grrs")?;
    cmd.arg("test").arg(file.path());
    cmd.assert()
        .success()
        .stdout(predicate::str::contains("A test\nAnother test"));

    Ok(())
}

#[test]
fn empty_file() -> Result<(), Box<dyn std::error::Error>> {
    let file = assert_fs::NamedTempFile::new("empty.txt")?;
    file.write_str("")?;

    let mut cmd = Command::cargo_bin("grrs")?;
    cmd.arg("pattern").arg(file.path());
    cmd.assert()
        .success()
        .stdout(predicate::str::is_empty());

    Ok(())
}

#[test]
fn pattern_not_found_in_file() -> Result<(), Box<dyn std::error::Error>> {
    let file = assert_fs::NamedTempFile::new("sample.txt")?;
    file.write_str("A test\nActual content\nMore content\nAnother test")?;

    let mut cmd = Command::cargo_bin("grrs")?;
    cmd.arg("nonexistent").arg(file.path());
    cmd.assert()
        .success()
        .stdout(predicate::str::is_empty());

    Ok(())
}


// #[test]
// fn multiple_matches_in_file() -> Result<(), Box<dyn std::error::Error>> {
//     let file = assert_fs::NamedTempFile::new("sample.txt")?;
//     file.write_str("Match here\nAnother match\nYet another match\nMatch here again")?;

//     let mut cmd = Command::cargo_bin("grrs")?;
//     cmd.arg("match").arg(file.path());
//     cmd.assert()
//         .success()
//         .stdout(predicate::str::contains("Match here")
//             .and(predicate::str::contains("Another match"))
//             .and(predicate::str::contains("Yet another match"))
//             .and(predicate::str::contains("Match here again")));

//     Ok(())
// }


#[test]
fn special_characters_in_file() -> Result<(), Box<dyn std::error::Error>> {
    let file = assert_fs::NamedTempFile::new("sample.txt")?;
    file.write_str("A special char test: !@#$%^&*()\nAnother line")?;

    let mut cmd = Command::cargo_bin("grrs")?;
    cmd.arg("!@#$%^&*()").arg(file.path());
    cmd.assert()
        .success()
        .stdout(predicate::str::contains("A special char test: !@#$%^&*()"));

    Ok(())
}