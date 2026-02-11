use std::str::FromStr;

use anyhow::Result;

use crate::mixal::number::Number;

use super::expression::Expression;

#[derive(Debug, PartialEq)]
pub struct Field {
    expression: Expression,
}

impl Field {
    /// Looks for an opening parenthesis in the given string, and parses the portion
    /// of the string from that index as a Field specifier, otherwise returns the
    /// default field specifier, which is equivalent to (0:5).
    pub fn find_field_or_default(s: &str, default: &str) -> Result<(Self, usize), anyhow::Error> {
        if let Some(idx) = s.find('(') {
            Ok((s[idx..].parse()?, idx))
        } else {
            Ok((
                Self {
                    expression: default.parse()?,
                },
                s.len(), // implicitly found at the "end" of the string
            ))
        }
    }
}

impl FromStr for Field {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if !(s.len() > 2 && s.starts_with('(') && s.ends_with(')')) {
            anyhow::bail!("Invalid field specifier: {}", s);
        }

        Ok(Field {
            expression: s[1..s.len() - 1].parse()?,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_from_str() {
        assert_eq!(
            "(1:4)".parse::<Field>().unwrap(),
            Field {
                expression: "1:4".parse().unwrap()
            }
        );
        assert_eq!(
            "(0:5)".parse::<Field>().unwrap(),
            Field {
                expression: "0:5".parse().unwrap()
            }
        );
        assert_eq!(
            "(1:1)".parse::<Field>().unwrap(),
            Field {
                expression: "1:1".parse().unwrap()
            }
        );
        assert!("(%invaliud%)".parse::<Field>().is_err());
        assert!("(".parse::<Field>().is_err());
        assert!("".parse::<Field>().is_err());
        assert!(")".parse::<Field>().is_err());
        assert!("(()".parse::<Field>().is_err());
    }

    #[test]
    fn test_find_field_or_default() {
        assert_eq!(
            Field::find_field_or_default("3+4(1:1)", "0:5").unwrap(),
            (
                Field {
                    expression: "1:1".parse().unwrap()
                },
                3
            )
        );
        assert_eq!(
            Field::find_field_or_default("10024(3:5)", "4").unwrap(),
            (
                Field {
                    expression: "4".parse().unwrap()
                },
                5
            )
        );
        assert_eq!(
            Field::find_field_or_default("10024", "0:5").unwrap(),
            (
                Field {
                    expression: "0:5".parse().unwrap()
                },
                5
            )
        );
        assert_eq!(
            Field::find_field_or_default("", "0:5").unwrap(),
            (
                Field {
                    expression: "0:5".parse().unwrap()
                },
                0
            )
        );
        assert!(Field::find_field_or_default("(%invalid%)", "0:5").is_err());
        assert!(Field::find_field_or_default("", "0:").is_err());
    }
}
