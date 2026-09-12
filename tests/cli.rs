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
