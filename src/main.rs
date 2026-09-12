//! Command-line interface for `typst2docx`.
use clap::Parser;
use std::path::PathBuf;

/// Convert Typst markup documents to Microsoft Word (.docx).
#[derive(Parser, Debug)]
#[command(version, about)]
struct Cli {
    /// Path to the input Typst (.typ) file
    input: PathBuf,

    /// Path to the output DOCX (.docx) file [default: <input>.docx]
    #[arg(short, long)]
    output: Option<PathBuf>,
}

fn main() {
    let args = Cli::parse();

    // Call into our core library.
    // Notice args.output.as_deref() converts Option<PathBuf> into Option<&Path>
    if let Err(err) = typst2docx::run(&args.input, args.output.as_deref()) {
        eprintln!("Error: {err}");
        std::process::exit(1);
    }
}
