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
pub fn run(input: &Path, output: Option<&Path>) -> Result<(), Typst2DocxError> {
    if !input.exists() {
        return Err(Typst2DocxError::InputFileNotFound(input.to_path_buf()));
    }

    let _output_path = resolve_output_path(input, output);
    Ok(())
}

/// Resolves the destination output path for the converted DOCX file.
///
/// If an explicit output path is provided (`Some`), it is returned.
/// If `None` is provided, the input file's extension is replaced with `.docx`.
#[must_use]
pub fn resolve_output_path(input: &Path, output: Option<&Path>) -> PathBuf {
    if let Some(explicit_path) = output {
        explicit_path.to_path_buf()
    } else {
        input.with_extension("docx")
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::path::Path;

    #[test]
    fn test_resolve_output_path_with_explicit_output() {
        let input = Path::new("documents/report.typ");
        let explicit = Path::new("custom/target.docx");

        let resolved = resolve_output_path(input, Some(explicit));
        assert_eq!(resolved, PathBuf::from("custom/target.docx"));
    }

    #[test]
    fn test_resolve_output_path_defaults_to_docx_extension() {
        let input = Path::new("documents/report.typ");
        let resolved = resolve_output_path(input, None);
        assert_eq!(resolved, PathBuf::from("documents/report.docx"));
    }
}
