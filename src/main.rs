mod plot;

use clap::Parser;
use clap::ValueEnum;
use std::fs::File;
use std::path::PathBuf;

use crate::plot::plot;

#[derive(Debug, Clone, ValueEnum)]
#[clap(rename_all = "lower")]
enum Mode {
    Plot,
}

#[derive(Debug, Clone, ValueEnum)]
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
        Mode::Plot => plot(file, out, &args.normalization)?,
    }
    Ok(())
}
