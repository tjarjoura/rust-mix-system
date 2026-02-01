use anyhow::Result;

use super::wval::WVal;

/// Pseudo-operation in MIXAL that sets the value of a symbol equal to
/// the given W-Value
pub struct Equ {
    wval: WVal,
}

impl Equ {
    /// Attempts to parse an EQU pseudo-operation from the input string.
    ///
    /// Returns:
    /// - `Ok(Some(equ))` — input starts with "EQU" and operand parsed successfully
    /// - `Ok(None)` — input does not start with "EQU", caller should try another parser
    /// - `Err(_)` — input starts with "EQU" but operand is invalid
    pub fn try_parse(input: &str) -> Result<Option<Self>> {
        let Some(rest) = input.strip_prefix("EQU") else {
            return Ok(None);
        };

        let operand = rest.trim();
        if operand.is_empty() {
            anyhow::bail!("Invalid EQU op: {}. Missing WVal", input);
        }

        let wval = operand.parse()?;
        Ok(Some(Equ { wval }))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_try_parse_valid() {
        let result = Equ::try_parse("EQU 1000").unwrap();
        assert!(result.is_some());
    }

    #[test]
    fn test_try_parse_valid_with_expression() {
        let result = Equ::try_parse("EQU X+1").unwrap();
        assert!(result.is_some());
    }

    #[test]
    fn test_try_parse_not_equ() {
        // Should return Ok(None) for other opcodes
        assert!(Equ::try_parse("ORIG 3000").unwrap().is_none());
        assert!(Equ::try_parse("CON 5").unwrap().is_none());
        assert!(Equ::try_parse("STJ EXIT").unwrap().is_none());
    }

    #[test]
    fn test_try_parse_missing_operand() {
        // Should error, not return None
        assert!(Equ::try_parse("EQU").is_err());
        assert!(Equ::try_parse("EQU   ").is_err());
    }

    #[test]
    fn test_try_parse_invalid_operand() {
        // Should error due to invalid WVal
        assert!(Equ::try_parse("EQU (bad)").is_err());
    }
}
