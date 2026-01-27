use std::str::FromStr;

use anyhow::{Result, anyhow, bail};

/// A Symbol represents a string of characters in the MIXAL assembly language that can
/// "stand for" a raw numerical value. These will be replaced with the underlying values
/// when assembled into MIX machine code.
/// The parsing rules are as laid out in TAOCP Vol. I, p. 153
#[derive(Debug, PartialEq)]
pub struct Symbol(pub String);
impl Symbol {
    pub const MAX_LENGTH: usize = 10;
    pub fn new(s: &str) -> Result<Self, anyhow::Error> {
        // TODO: It may make sense to create a proper error type, but using anyhow for now
        if s.is_empty() {
            return Err(anyhow!("Cannot construct symbol from empty string"));
        }

        if s.len() > Self::MAX_LENGTH {
            bail!(
                "Symbol '{}' exceeds maximum length of {} characters",
                s,
                Self::MAX_LENGTH
            );
        }

        if !s.chars().all(Self::is_valid_char) {
            bail!("Invalid characters found in symbol '{}'", s);
        }

        if !s.chars().any(|c| c.is_alphabetic()) {
            bail!("A symbol must contain at least one letter");
        }

        Ok(Symbol(s.to_string()))
    }

    // The MIX Computer only supports a small subset of our modern ASCII and Unicode
    // character sets. While we could expand the MIXAL assembly language to take
    // advantage of the additional characters available on modern machines, I thought
    // it would be more interesting to be historically authentic. Therefore, only
    // uppercase characters are allowed.
    fn is_valid_char(c: char) -> bool {
        matches!(c, 'A'..='Z' | '0'..='9')
    }
}

impl FromStr for Symbol {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        Symbol::new(s)
    }
}

// TODO: Move this to its own module
// Next there is some notion of a "W-Value" -- this goes in the "address" field of  pseudo-operations,
// The "W" stands for "Word", so this is meant to represent a full word constant.
/// Represents a "line" of code in a MIXAL assembly program
pub struct MixalStatement {
    loc: Option<Symbol>,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new_symbol_valid() {
        assert!(Symbol::new("5H").is_ok());
        assert!(Symbol::new("MAXIMCHARS").is_ok());
    }

    #[test]
    fn test_new_symbol_empty() {
        assert!(Symbol::new("").is_err());
    }

    #[test]
    fn test_new_symbol_too_long() {
        assert!(Symbol::new("AAAAAAAAAAA").is_err());
    }

    #[test]
    fn test_new_symbol_no_letters() {
        assert!(Symbol::new("12345").is_err());
    }

    #[test]
    fn test_new_symbol_invalid_chars() {
        assert!(Symbol::new("abcd1").is_err());
        assert!(Symbol::new("LABEL@#").is_err());
        assert!(Symbol::new("LAbEL3").is_err());
    }
}
