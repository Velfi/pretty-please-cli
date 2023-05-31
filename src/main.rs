use clap::Parser;
use std::path::PathBuf;

/// Simple program to run prettyplease on Rust files
#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// Path of the rust file to format
    #[arg(short, long)]
    input: PathBuf,

    /// Path of the file the formatted code will be written to.
    #[arg(short, long)]
    output: Option<PathBuf>,
}

fn main() {
    let args = Args::parse();

    let f = std::fs::read_to_string(&args.input).unwrap();
    let syntax_tree = syn::parse_file(&f).unwrap();
    let formatted = prettyplease::unparse(&syntax_tree);

    std::fs::write(args.output.unwrap_or(args.input), formatted).unwrap();
}