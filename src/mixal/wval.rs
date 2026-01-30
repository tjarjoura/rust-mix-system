use anyhow::{Result, bail};
use std::str::FromStr;

use super::expression::Expression;

/// A "Word Value" in MIXAL. A sort of inline program, a sequence of expressions
/// and field lookups that eventually evaluate to a constant. Used with MIXAL
/// pseudo-operations, but not part of the machine language itself.
pub enum WVal {
    FutureRef(FutureRef),
    WValInner(WValInner),
}

impl FromStr for WVal {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if s.len() > 10 {
            anyhow::bail!("WValue too long, maximum 10 characters: {}", s)
        }
        if s.starts_with("=") && s.ends_with("=") {
            return Ok(WVal::FutureRef(s.parse()?));
        }

        todo!();
    }
}

/// A W-value that is wrapped in '=' signs stores the result of the value at a
/// location in memory and resolves to that address, rather than the result of the
/// value itself
struct FutureRef {
    wval: WValInner,
}

impl FromStr for FutureRef {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if !(s.starts_with("=") && s.ends_with("=") && s.len() > 2) {
            anyhow::bail!(
                "Invalid WValue future reference, missing wrapping '=': {}",
                s
            );
        }

        Ok(Self {
            wval: s[1..s.len() - 1].parse()?,
        })
    }
}

struct WValInner {
    components: Vec<WValComponent>,
}

impl FromStr for WValInner {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(WValInner {
            // parse each component individually, if any fail the collect::<Result<Vec>>
            // call will result in an error which is bubbled up by '?'.
            components: s
                .split(",")
                .map(|s| s.parse())
                .collect::<Result<Vec<_>, _>>()?,
        })
    }
}

/// Each component consists of an expression (which is evaluated normally) and a field
/// look up, which selects a portion of the expression result.
struct WValComponent {
    expression: Expression,
    // field: Field,
}

impl FromStr for WValComponent {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let split = s.split('(');
        todo!()
    }
}
