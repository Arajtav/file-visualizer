use std::path::PathBuf;

use clap::Parser;
use clap::ValueEnum;

#[derive(Debug, Clone, ValueEnum)]
#[clap(rename_all = "lower")]
enum Mode {}

#[derive(Parser, Debug)]
#[command(long_about = None)]
struct Args {
    /// Mode
    #[arg(short, long)]
    mode: Mode,

    /// Input file
    file: PathBuf,

    /// Output file
    out: PathBuf,
}

fn main() {
    let args = Args::parse();
}
