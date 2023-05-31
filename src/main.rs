use std::{fs, path::PathBuf};

use clap::Parser;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// Input markdown file to parse to HTML
    #[arg(short, long)]
    file: PathBuf,
}

fn main() {
    let args = Args::parse();
    let markdown = fs::read_to_string(args.file).expect("Unable to read file");
    let out = grime::transpile::parse(&markdown);
    println!("{out}");
}
