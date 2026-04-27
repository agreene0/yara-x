use std::fs;

use assert_cmd::{Command, cargo_bin};
use assert_fs::TempDir;
use assert_fs::prelude::*;
use predicates::prelude::*;

#[test]
fn fmt() {
    let temp_dir = TempDir::new().unwrap();
    let input_file = temp_dir.child("rule.yar");

    input_file.write_str("rule test { condition: true }").unwrap();

    Command::new(cargo_bin!("yr"))
        .arg("fmt")
        .arg(input_file.path())
        .assert()
        .code(1); // Exit code 1 indicates that the file was modified.

    Command::new(cargo_bin!("yr"))
        .arg("fmt")
        .arg(input_file.path())
        .assert()
        .code(0); // Second time that we format the same file, no expected changes.
}

#[test]
fn fmt_check_shows_filenames() {
    let temp_dir = TempDir::new().unwrap();
    let input_file = temp_dir.child("rule.yar");

    input_file.write_str("rule test { condition: true }").unwrap();

    Command::new(cargo_bin!("yr"))
        .arg("fmt")
        .arg("--check")
        .arg(input_file.path())
        .assert()
        .stderr(predicate::str::contains("rule.yar"))
        .code(1);
}

#[test]
fn utf8_error() {
    let temp_dir = TempDir::new().unwrap();
    let input_file = temp_dir.child("rule.yar");

    input_file.write_binary(&[0xff, 0xff]).unwrap();

    Command::new(cargo_bin!("yr"))
        .arg("fmt")
        .arg(input_file.path())
        .assert()
        .stderr("error: invalid UTF-8 at [0..1]\n")
        .code(1);
}

#[test]
fn fmt_tab_size() {
    let temp_dir = TempDir::new().unwrap();
    let input_file = temp_dir.child("rule.yar");

    input_file.write_str("rule test { condition: true }").unwrap();

    // Format with --tab-size 4: the output indentation should use 4 spaces.
    Command::new(cargo_bin!("yr"))
        .arg("fmt")
        .arg("--tab-size")
        .arg("4")
        .arg(input_file.path())
        .assert()
        .code(1); // File was modified.

    // With --tab-size 4, condition: should be indented with 4 spaces.
    let contents = fs::read_to_string(input_file.path()).unwrap();
    assert_eq!(
        contents,
        "rule test {\n    condition:\n        true\n}\n",
        "Expected 4-space indentation with --tab-size 4, got:\n{}",
        contents
    );

    // Second run with the same --tab-size should make no further changes.
    Command::new(cargo_bin!("yr"))
        .arg("fmt")
        .arg("--tab-size")
        .arg("4")
        .arg(input_file.path())
        .assert()
        .code(0);
}

#[test]
fn fmt_tab_indented_input() {
    let temp_dir = TempDir::new().unwrap();
    let input_file = temp_dir.child("rule.yar");

    // Using tab characters for indentation.
    input_file
        .write_str("rule test {\n\tcondition:\n\t\ttrue\n}\n")
        .unwrap();

    // Tabs in the input should be converted to spaces and the output should
    // use 4-space indentation.
    Command::new(cargo_bin!("yr"))
        .arg("fmt")
        .arg("--tab-size")
        .arg("4")
        .arg(input_file.path())
        .assert()
        .code(1); // File was modified (tabs converted to spaces).

    let contents = fs::read_to_string(input_file.path()).unwrap();

    assert!(
        !contents.contains('\t'),
        "Expected no tab characters in formatted output, got:\n{}",
        contents
    );

    assert_eq!(
        contents,
        "rule test {\n    condition:\n        true\n}\n",
        "Expected 4-space indented output, got:\n{}",
        contents
    );

    // Second run should make no further changes.
    Command::new(cargo_bin!("yr"))
        .arg("fmt")
        .arg("--tab-size")
        .arg("4")
        .arg(input_file.path())
        .assert()
        .code(0);
}
