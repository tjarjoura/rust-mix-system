use std::str::FromStr;

use anyhow::Result;

use super::symbol::Symbol;

pub enum MixInstruction {}
pub enum Operation {
    Instruction(MixInstruction),
    Equ,
    Orig,
    Con,
    Alf,
    End,
}

impl FromStr for Operation {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        todo!();
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
