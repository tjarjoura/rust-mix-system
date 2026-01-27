use std::str::FromStr;

use crate::mixal::operator::UnaryOperator;

use super::number::Number;
use super::operator::BinaryOperator;
use super::symbol::Symbol;

/// Represents an expression in MIXAL assembly language
/// An expression is either:
///     An asterisk "*" -- which means the current memory location the assembler is writing to
///     A symbol -- which is a name that points to some other value
///     A number -- which is a string of at most 10 digits
///     A binary or unary operator -- which recursively contain expressions that the operators act on
#[derive(Debug, PartialEq)]
enum Expression {
    Asterisk,
    Symbol(Symbol),
    Number(Number),
    BinaryOperation(BinaryOperator, Box<Expression>, Box<Expression>),
    UnaryOperation(UnaryOperator, Box<Expression>),
}

impl FromStr for Expression {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if s == "*" {
            Ok(Expression::Asterisk)
        } else if UnaryOperator::starts_with(s) {
            Ok(Expression::UnaryOperation(
                s[0..1].parse()?,
                Box::new(s[1..].parse()?),
            ))
        } else if let Some((pos, op)) = BinaryOperator::find_leftmost_in(s) {
            Ok(Expression::BinaryOperation(
                s[pos..pos + op.len()].parse()?,
                Box::new(s[0..pos].parse()?),
                Box::new(s[pos + 1..].parse()?),
            ))
        } else if s.chars().all(|c| c.is_ascii_digit()) {
            Ok(Expression::Number(s.parse()?))
        } else {
            Ok(Expression::Symbol(s.parse()?))
        }
    }
}

#[cfg(test)]
mod tests {
    pub use super::*;

    #[test]
    fn test_from_str() {
        assert_eq!(
            "34".parse::<Expression>().unwrap(),
            Expression::Number(Number(34))
        );
        assert_eq!(
            "34+7".parse::<Expression>().unwrap(),
            Expression::BinaryOperation(
                BinaryOperator::Plus,
                Box::new(Expression::Number(Number(34))),
                Box::new(Expression::Number(Number(7)))
            )
        );
        assert_eq!(
            "34+7-4".parse::<Expression>().unwrap(),
            Expression::BinaryOperation(
                BinaryOperator::Plus,
                Box::new(Expression::Number(Number(34))),
                Box::new(Expression::BinaryOperation(
                    BinaryOperator::Minus,
                    Box::new(Expression::Number(Number(7))),
                    Box::new(Expression::Number(Number(4))),
                ))
            )
        );
        assert_eq!(
            "-34+7-4".parse::<Expression>().unwrap(),
            Expression::UnaryOperation(
                UnaryOperator::Minus,
                Box::new(Expression::BinaryOperation(
                    BinaryOperator::Plus,
                    Box::new(Expression::Number(Number(34))),
                    Box::new(Expression::BinaryOperation(
                        BinaryOperator::Minus,
                        Box::new(Expression::Number(Number(7))),
                        Box::new(Expression::Number(Number(4))),
                    ))
                ))
            )
        );
    }
}
