//! Core library for converting Typst documents to DOCX.

use std::path::{Path, PathBuf};
use thiserror::Error;

/// Domain errors that can occur during document conversion.
#[derive(Error, Debug)]
pub enum Typst2DocxError {
    /// An underlying I/O error occurred.
    #[error("I/O error: {0}")]
    Io(#[from] std::io::Error),

    /// The specified input Typst file was not found.
    #[error("Input file not found: {0}")]
    InputFileNotFound(PathBuf),
}

/// Runs the conversion pipeline.
///
/// # Errors
///
/// Returns [`Typst2DocxError::InputFileNotFound`] if the input file does not exist.
pub fn run(input: &Path, _output: Option<&Path>) -> Result<(), Typst2DocxError> {
    if !input.exists() {
        return Err(Typst2DocxError::InputFileNotFound(input.to_path_buf()));
    }

    Ok(())
}
