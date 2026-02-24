use std::{
    collections::HashMap,
    fs::File,
    io::{BufRead, BufReader},
};

use super::expression::Expression;
use super::statement::Statement;
use anyhow::Result;

// A MIX machine consists of 4000 machine words which are each represented as 6 u8 bytes.
const N_WORDS: usize = 4000;
const BYTES_PER_WORD: usize = 6;

pub struct MachineWord {
    bytes: [u8; BYTES_PER_WORD],
    symbols: HashMap<String, Expression>,
}

pub struct AssemblerState {
    output: [MachineWord; N_WORDS],
}
/// Main entrypoint for assembling a file -- subject to change
pub fn assemble_file(path: &str) -> Result<()> {
    let inp_file = File::open(path)?;
    let reader = BufReader::new(inp_file);
    let statements: Result<Vec<Statement>> = reader
        .lines()
        .map(|line| line?.parse::<Statement>())
        .collect();

    println!("Parsed {} statements", statements?.len());
    Ok(())
}
