#![allow(missing_docs)]

use assert_cmd::Command;
use predicates::prelude::*;

#[test]
fn test_cli_missing_input_argument_fails() {
    let mut cmd = Command::cargo_bin("typst2docx").expect("Failed to find binary");
    cmd.assert()
        .failure()
        .stderr(predicate::str::contains("Usage"));
}

#[test]
fn test_cli_input_file_not_found_fails() {
    let mut cmd = Command::cargo_bin("typst2docx").expect("binary should exist");
    cmd.arg("nonexistent_document_12345.typ")
        .assert()
        .failure()
        .stderr(
            predicate::str::contains("not found").or(predicate::str::contains("does not exist")),
        );
}
