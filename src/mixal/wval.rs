use anyhow::{Result, bail};
use std::str::FromStr;

use crate::mixal::field::Field;

use super::expression::Expression;

/// A "Word Value" in MIXAL. A sort of inline program, a sequence of expressions
/// and field lookups that eventually evaluate to a constant. Used with MIXAL
/// pseudo-operations, but not part of the machine language itself.
#[derive(Debug, PartialEq)]
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

        Ok(WVal::WValInner(s.parse()?))
    }
}

/// A W-value that is wrapped in '=' signs stores the result of the value at a
/// location in memory and resolves to that address, rather than the result of the
/// value itself
#[derive(Debug, PartialEq)]
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

#[derive(Debug, PartialEq)]
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
#[derive(Debug, PartialEq)]
struct WValComponent {
    expression: Expression,
    field: Field,
}

impl FromStr for WValComponent {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (field, idx) = Field::find_field_or_default(s, "0:5")?;
        Ok(WValComponent {
            expression: s[0..idx].parse()?,
            field,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn component(expr: &str, field: &str) -> WValComponent {
        WValComponent {
            expression: expr.parse().unwrap(),
            field: field.parse().unwrap(),
        }
    }

    fn component_default_field(expr: &str) -> WValComponent {
        component(expr, "(0:5)")
    }

    #[test]
    fn test_wval_component_with_field() {
        assert_eq!(
            "3+4(1:1)".parse::<WValComponent>().unwrap(),
            component("3+4", "(1:1)")
        );
        assert_eq!(
            "10024(3:5)".parse::<WValComponent>().unwrap(),
            component("10024", "(3:5)")
        );
        assert_eq!(
            "*(0:2)".parse::<WValComponent>().unwrap(),
            component("*", "(0:2)")
        );
        assert_eq!(
            "LABEL(1:3)".parse::<WValComponent>().unwrap(),
            component("LABEL", "(1:3)")
        );
    }

    #[test]
    fn test_wval_component_default_field() {
        assert_eq!(
            "10024".parse::<WValComponent>().unwrap(),
            component_default_field("10024")
        );
        assert_eq!(
            "5".parse::<WValComponent>().unwrap(),
            component_default_field("5")
        );
        assert_eq!(
            "*".parse::<WValComponent>().unwrap(),
            component_default_field("*")
        );
        assert_eq!(
            "X+1".parse::<WValComponent>().unwrap(),
            component_default_field("X+1")
        );
    }

    #[test]
    fn test_wval_component_invalid() {
        // Invalid field specifier
        assert!("5(%bad%)".parse::<WValComponent>().is_err());
        // Empty string results in empty expression which fails
        assert!("".parse::<WValComponent>().is_err());
        // Only field specifier, no expression
        assert!("(1:1)".parse::<WValComponent>().is_err());
    }

    fn wval_inner(components: Vec<WValComponent>) -> WValInner {
        WValInner { components }
    }

    #[test]
    fn test_wval_inner_single_component() {
        assert_eq!(
            "5".parse::<WValInner>().unwrap(),
            wval_inner(vec![component_default_field("5")])
        );
        assert_eq!(
            "X(1:3)".parse::<WValInner>().unwrap(),
            wval_inner(vec![component("X", "(1:3)")])
        );
    }

    #[test]
    fn test_wval_inner_multiple_components() {
        assert_eq!(
            "1,2,3".parse::<WValInner>().unwrap(),
            wval_inner(vec![
                component_default_field("1"),
                component_default_field("2"),
                component_default_field("3"),
            ])
        );
        assert_eq!(
            "X(1:1),Y(2:2)".parse::<WValInner>().unwrap(),
            wval_inner(vec![component("X", "(1:1)"), component("Y", "(2:2)"),])
        );
        assert_eq!(
            "1(0:1),2(1:2),3(2:3)".parse::<WValInner>().unwrap(),
            wval_inner(vec![
                component("1", "(0:1)"),
                component("2", "(1:2)"),
                component("3", "(2:3)"),
            ])
        );
    }

    #[test]
    fn test_wval_regular() {
        assert_eq!(
            "5".parse::<WVal>().unwrap(),
            WVal::WValInner(wval_inner(vec![component_default_field("5")]))
        );
        assert_eq!(
            "1,2".parse::<WVal>().unwrap(),
            WVal::WValInner(wval_inner(vec![
                component_default_field("1"),
                component_default_field("2"),
            ]))
        );
    }

    #[test]
    fn test_wval_future_ref() {
        assert_eq!(
            "=5=".parse::<WVal>().unwrap(),
            WVal::FutureRef(FutureRef {
                wval: wval_inner(vec![component_default_field("5")])
            })
        );
        assert_eq!(
            "=1,2=".parse::<WVal>().unwrap(),
            WVal::FutureRef(FutureRef {
                wval: wval_inner(vec![
                    component_default_field("1"),
                    component_default_field("2"),
                ])
            })
        );
        assert_eq!(
            "=X(1:3)=".parse::<WVal>().unwrap(),
            WVal::FutureRef(FutureRef {
                wval: wval_inner(vec![component("X", "(1:3)")])
            })
        );
    }

    #[test]
    fn test_wval_invalid() {
        // Too long (over 10 characters)
        assert!("12345678901".parse::<WVal>().is_err());
        // Malformed future ref (missing closing =)
        assert!("=5".parse::<WVal>().is_err());
    }
}
