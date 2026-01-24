use std::fs::File;
use std::io::{BufRead, BufReader};

use anyhow::Result;
use clap::Parser;

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
    println!("Input: {}", cli.input);

    let inp_file = File::open(cli.input)?;
    let reader = BufReader::new(inp_file);
    for line in reader.lines() {
        println!("Read line: {}", line?);
    }

    Ok(())
}
