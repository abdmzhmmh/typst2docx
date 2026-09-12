//! Command-line interface for `typst2docx`.
use clap::Parser;
use std::path::PathBuf;

#[derive(Parser, Debug)]
#[command(version, about)]
struct Cli {
    input: PathBuf,
    #[arg(short, long)]
    output: Option<PathBuf>,
}

fn main() {
    let _args = Cli::parse();
}
