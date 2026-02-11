use super::expression::Expression;
use super::field::Field;

#[derive(Debug, PartialEq)]
pub enum Sign {
    Positive,
    Negative,
}

#[derive(Debug, PartialEq)]
pub struct Address {
    pub sign: Sign,
    pub address: Expression,
    pub index: Expression,
    pub field: Field,
}

impl Address {
    pub fn new_with_default_field(s: &str, default_field: &str) -> anyhow::Result<Self> {
        let (sign, rest) = if s.starts_with('-') {
            (Sign::Negative, &s[1..])
        } else if s.starts_with('+') {
            (Sign::Positive, &s[1..])
        } else {
            (Sign::Positive, s)
        };

        // look for field specifier
        let (field, field_pos) = Field::find_field_or_default(rest, default_field)?;

        // look for index
        let (index, index_pos) = if let Some(index_pos) = rest[..field_pos].find(",") {
            (rest[index_pos + 1..field_pos].parse()?, index_pos)
        } else {
            ("0".parse()?, field_pos)
        };

        // parse base address
        let address = if rest[..index_pos].is_empty() {
            "0".parse()?
        } else {
            rest[..index_pos].parse()?
        };

        Ok(Self {
            sign,
            address,
            index,
            field,
        })
    }
}

/// Represents a MIX machine instruction to be assembled
pub struct MixInstruction {
    pub operation_code: u8,
}

impl MixInstruction {
    /// Returns Some(MixInstruction) if opcode is recognized, None otherwise
    /// If opcode is recognized but there is an error in parsing, an error is
    /// returned
    pub fn try_parse(opcode: &str, rest: &str) -> anyhow::Result<Option<Self>> {
        let (operation_code, default_field) = match opcode {
            "NOP" => (0, "0:5"),
            _ => {
                return Ok(None);
            }
        };

        todo!()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn addr(s: &str, default_field: &str) -> Address {
        Address::new_with_default_field(s, default_field).unwrap()
    }

    fn expr(s: &str) -> Expression {
        s.parse().unwrap()
    }

    fn field(s: &str) -> Field {
        s.parse().unwrap()
    }

    #[test]
    fn test_address_full() {
        assert_eq!(
            addr("2000,2(0:3)", "0:5"),
            Address {
                sign: Sign::Positive,
                address: expr("2000"),
                index: expr("2"),
                field: field("(0:3)"),
            }
        );
    }

    #[test]
    fn test_address_no_field() {
        assert_eq!(
            addr("2000,2", "0:5"),
            Address {
                sign: Sign::Positive,
                address: expr("2000"),
                index: expr("2"),
                field: field("(0:5)"),
            }
        );
    }

    #[test]
    fn test_address_no_index() {
        assert_eq!(
            addr("2000(1:3)", "0:5"),
            Address {
                sign: Sign::Positive,
                address: expr("2000"),
                index: expr("0"),
                field: field("(1:3)"),
            }
        );
    }

    #[test]
    fn test_address_only() {
        assert_eq!(
            addr("2000", "0:5"),
            Address {
                sign: Sign::Positive,
                address: expr("2000"),
                index: expr("0"),
                field: field("(0:5)"),
            }
        );
    }

    #[test]
    fn test_address_negative_sign() {
        assert_eq!(
            addr("-2000,2(0:3)", "0:5"),
            Address {
                sign: Sign::Negative,
                address: expr("2000"),
                index: expr("2"),
                field: field("(0:3)"),
            }
        );
    }

    #[test]
    fn test_address_positive_sign() {
        assert_eq!(
            addr("+2000", "0:5"),
            Address {
                sign: Sign::Positive,
                address: expr("2000"),
                index: expr("0"),
                field: field("(0:5)"),
            }
        );
    }

    #[test]
    fn test_address_empty() {
        assert_eq!(
            addr("", "0:5"),
            Address {
                sign: Sign::Positive,
                address: expr("0"),
                index: expr("0"),
                field: field("(0:5)"),
            }
        );
    }

    #[test]
    fn test_address_with_symbols() {
        assert_eq!(
            addr("LABEL,2(1:3)", "0:5"),
            Address {
                sign: Sign::Positive,
                address: expr("LABEL"),
                index: expr("2"),
                field: field("(1:3)"),
            }
        );
    }

    #[test]
    fn test_address_with_expression() {
        assert_eq!(
            addr("2000+5,3", "0:5"),
            Address {
                sign: Sign::Positive,
                address: expr("2000+5"),
                index: expr("3"),
                field: field("(0:5)"),
            }
        );
    }

    #[test]
    fn test_address_different_default_field() {
        assert_eq!(
            addr("2000", "1:3"),
            Address {
                sign: Sign::Positive,
                address: expr("2000"),
                index: expr("0"),
                field: field("(1:3)"),
            }
        );
    }
}
