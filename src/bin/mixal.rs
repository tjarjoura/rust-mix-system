use std::fs::File;
use std::io::{BufRead, BufReader};

use anyhow::Result;
use clap::Parser;

use mix_system::mixal::assemble::assemble_file;

#[derive(Parser)]
#[command(name = "mixal")]
#[command(about = "A MIX assembler")]
struct Cli {
    /// Input file containing MIX assembly code
    #[arg(short, long)]
    input: String,
}

fn main() -> Result<()> {
    let cli = Cli::parse();
    assemble_file(&cli.input)
}
