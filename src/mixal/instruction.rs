use crate::mixal::wval::WVal;

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
    pub address: WVal,
    pub index: Expression,
    pub field: Field,
}

impl Address {
    pub fn new_with_default_field(s: &str, default_field: &str) -> anyhow::Result<Self> {
        // how to handle trailing white space?
        // we could just strip any trailing whitespace at the higher layer
        // I was worried about ALF, but that only cares about leading whitespace, not trailing
        // but if the char data includes trailing whitespaces, that would be stripped off
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
    pub address: Address,
}

impl MixInstruction {
    /// Returns Some(MixInstruction) if opcode is recognized, None otherwise
    /// If opcode is recognized but there is an error in parsing, an error is
    /// returned
    pub fn try_parse(opcode: &str, rest: &str) -> anyhow::Result<Self> {
        let (operation_code, default_field) = match opcode {
            "NOP" => (0, "0:5"),
            "ADD" => (1, "0:5"),
            "FADD" => (1, "6"),
            "SUB" => (2, "0:5"),
            "FSUB" => (2, "6"),
            "MUL" => (3, "0:5"),
            "FMUL" => (3, "6"),
            "DIV" => (4, "0:5"),
            "FDIV" => (4, "6"),
            "NUM" => (5, "0"),
            "CHAR" => (5, "1"),
            "HLT" => (5, "2"),
            "SLA" => (6, "0"),
            "SRA" => (6, "1"),
            "SLAX" => (6, "2"),
            "SRAX" => (6, "3"),
            "SLC" => (6, "4"),
            "SRC" => (6, "5"),
            "MOVE" => (7, "1"),
            "LDA" => (8, "0:5"),
            "LD1" => (9, "0:5"),
            "LD2" => (10, "0:5"),
            "LD3" => (11, "0:5"),
            "LD4" => (12, "0:5"),
            "LD5" => (13, "0:5"),
            "LD6" => (14, "0:5"),
            "LDX" => (15, "0:5"),
            "LDAN" => (16, "0:5"),
            "LD1N" => (17, "0:5"),
            "LD2N" => (18, "0:5"),
            "LD3N" => (19, "0:5"),
            "LD4N" => (20, "0:5"),
            "LD5N" => (21, "0:5"),
            "LD6N" => (22, "0:5"),
            "LDXN" => (23, "0:5"),
            "STA" => (24, "0:5"),
            "ST1" => (25, "0:5"),
            "ST2" => (26, "0:5"),
            "ST3" => (27, "0:5"),
            "ST4" => (28, "0:5"),
            "ST5" => (29, "0:5"),
            "ST6" => (30, "0:5"),
            "STX" => (31, "0:5"),
            "STJ" => (32, "0:2"),
            "STZ" => (33, "0:5"),
            "JBUS" => (34, "0"),
            "IOC" => (35, "0"),
            "IN" => (36, "0"),
            "OUT" => (37, "0"),
            "JRED" => (38, "0"),
            "JMP" => (39, "0"),
            "JSJ" => (39, "1"),
            "JOV" => (39, "2"),
            "JNOV" => (39, "3"),
            "JL" => (39, "4"),
            "JE" => (39, "5"),
            "JG" => (39, "6"),
            "JGE" => (39, "7"),
            "JNE" => (39, "8"),
            "JLE" => (39, "9"),
            "JAN" => (40, "0"),
            "JAZ" => (40, "1"),
            "JAP" => (40, "2"),
            "JANN" => (40, "3"),
            "JANZ" => (40, "4"),
            "JANP" => (40, "5"),
            "J1N" => (41, "0"),
            "J1Z" => (41, "1"),
            "J1P" => (41, "2"),
            "J1NN" => (41, "3"),
            "J1NZ" => (41, "4"),
            "J1NP" => (41, "5"),
            "J2N" => (42, "0"),
            "J2Z" => (42, "1"),
            "J2P" => (42, "2"),
            "J2NN" => (42, "3"),
            "J2NZ" => (42, "4"),
            "J2NP" => (42, "5"),
            "J3N" => (43, "0"),
            "J3Z" => (43, "1"),
            "J3P" => (43, "2"),
            "J3NN" => (43, "3"),
            "J3NZ" => (43, "4"),
            "J3NP" => (43, "5"),
            "J4N" => (44, "0"),
            "J4Z" => (44, "1"),
            "J4P" => (44, "2"),
            "J4NN" => (44, "3"),
            "J4NZ" => (44, "4"),
            "J4NP" => (44, "5"),
            "J5N" => (45, "0"),
            "J5Z" => (45, "1"),
            "J5P" => (45, "2"),
            "J5NN" => (45, "3"),
            "J5NZ" => (45, "4"),
            "J5NP" => (45, "5"),
            "J6N" => (46, "0"),
            "J6Z" => (46, "1"),
            "J6P" => (46, "2"),
            "J6NN" => (46, "3"),
            "J6NZ" => (46, "4"),
            "J6NP" => (46, "5"),
            "JXN" => (47, "0"),
            "JXZ" => (47, "1"),
            "JXP" => (47, "2"),
            "JXNN" => (47, "3"),
            "JXNZ" => (47, "4"),
            "JXNP" => (47, "5"),
            "INCA" => (48, "0"),
            "DECA" => (48, "1"),
            "ENTA" => (48, "2"),
            "ENNA" => (48, "3"),
            "INC1" => (49, "0"),
            "DEC1" => (49, "1"),
            "ENT1" => (49, "2"),
            "ENN1" => (49, "3"),
            "INC2" => (50, "0"),
            "DEC2" => (50, "1"),
            "ENT2" => (50, "2"),
            "ENN2" => (50, "3"),
            "INC3" => (51, "0"),
            "DEC3" => (51, "1"),
            "ENT3" => (51, "2"),
            "ENN3" => (51, "3"),
            "INC4" => (52, "0"),
            "DEC4" => (52, "1"),
            "ENT4" => (52, "2"),
            "ENN4" => (52, "3"),
            "INC5" => (53, "0"),
            "DEC5" => (53, "1"),
            "ENT5" => (53, "2"),
            "ENN5" => (53, "3"),
            "INC6" => (54, "0"),
            "DEC6" => (54, "1"),
            "ENT6" => (54, "2"),
            "ENN6" => (54, "3"),
            "INCX" => (55, "0"),
            "DECX" => (55, "1"),
            "ENTX" => (55, "2"),
            "ENNX" => (55, "3"),
            "CMPA" => (56, "0:5"),
            "FCMP" => (56, "6"),
            "CMP1" => (57, "0:5"),
            "CMP2" => (58, "0:5"),
            "CMP3" => (59, "0:5"),
            "CMP4" => (60, "0:5"),
            "CMP5" => (61, "0:5"),
            "CMP6" => (62, "0:5"),
            "CMPX" => (63, "0:5"),
            _ => {
                anyhow::bail!("Unrecognized opcode: {}", opcode);
            }
        };

        let address = Address::new_with_default_field(rest, default_field)?;
        Ok(MixInstruction {
            operation_code,
            address,
        })
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

    fn wval(s: &str) -> WVal {
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
                address: wval("2000"),
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
                address: wval("2000"),
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
                address: wval("2000"),
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
                address: wval("2000"),
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
                address: wval("2000"),
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
                address: wval("2000"),
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
                address: wval("0"),
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
                address: wval("LABEL"),
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
                address: wval("2000+5"),
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
                address: wval("2000"),
                index: expr("0"),
                field: field("(1:3)"),
            }
        );
    }
}
