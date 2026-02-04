use std::str::FromStr;

use anyhow::Result;

use super::equ::Equ;
use super::symbol::Symbol;

pub enum MixInstruction {}
pub enum Operation {
    Instruction(MixInstruction),
    Equ(Equ),
    Orig,
    Con,
    Alf,
    End,
}

impl FromStr for Operation {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        // Some operations need an operand, some can just be an opcode
        // We should just pass the remainder, whether it is an empty string or contains the operand,
        // and let the constructor handle the empty string case
        let (opcode, rest) = s.split_once(char::is_whitespace).unwrap_or((s, ""));

        match opcode {
            "EQU" => Ok(Operation::Equ(Equ{wval: rest.parse()?})),
            _ => {
                anyhow::bail!("Unrecognized OPCODE: {}", opcode);
            }
        }
}

/// Corresponds to one line of input in a MIXAL program
pub struct Statement {
    loc: Option<Symbol>,
    op: Operation,
}

impl FromStr for Statement {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        // Each line of a Mixal program can contain an optional LOC field, which is
        // determined by whether or not the line begins with whitespace (thus skipping
        // the LOC field) or not
        let (loc, opstr) = if !s.starts_with(char::is_whitespace) {
            if let Some((before, after)) = s.split_once(char::is_whitespace) {
                (Some(before.parse()?), after.trim_start())
            } else {
                // If we get here, we had a character in the symbol field but no corresponding OP
                // field, which is an invalid statement
                anyhow::bail!("Missing OP field after LOC field in: {}", s);
            }
        } else {
            // No LOC field, so proceed with parsing the entire s (after leading whitspace) as an OP field
            (None, s.trim_start())
        };

        Ok(Statement {
            loc: loc,
            op: opstr.parse()?,
        })
    }
}
