mod adjacency;
mod utils;

use clap::Parser;
use clap::ValueEnum;
use std::fs::File;
use std::path::PathBuf;

use crate::adjacency::adjacency;

#[derive(Debug, Clone, ValueEnum)]
#[clap(rename_all = "lower")]
enum Mode {
    Adjacency,
}

#[derive(Debug, Clone, ValueEnum, Copy)]
#[clap(rename_all = "kebab-case")]
enum Normalization {
    Max,
    MinMax,
}

#[derive(Parser, Debug)]
#[command(long_about = None)]
struct Args {
    /// Mode
    mode: Mode,

    /// Normalization method
    #[arg(long, default_value = "max")]
    normalization: Normalization,

    /// Whether to, during the normalization, ignore the most frequent value.
    #[arg(long, short, default_value_t = false)]
    ignore_most_frequent: bool,

    /// Input file
    file: PathBuf,

    /// Output file
    out: PathBuf,
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args = Args::parse();

    let file = File::open(&args.file)?;
    let out = File::create(&args.out)?;

    match args.mode {
        Mode::Adjacency => adjacency(file, out, args.normalization, args.ignore_most_frequent)?,
    }
    Ok(())
}
