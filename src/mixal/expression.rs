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
        } else if let Some((pos, op)) = BinaryOperator::find_rightmost_in(s) {
            Ok(Expression::BinaryOperation(
                s[pos..pos + op.len()].parse()?,
                Box::new(s[0..pos].parse()?),
                Box::new(s[pos + 1..].parse()?),
            ))
        } else if UnaryOperator::starts_with(s) {
            Ok(Expression::UnaryOperation(
                s[0..1].parse()?,
                Box::new(s[1..].parse()?),
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
    use super::*;

    fn num(n: u32) -> Expression {
        Expression::Number(Number(n))
    }

    fn binop(op: BinaryOperator, left: Expression, right: Expression) -> Expression {
        Expression::BinaryOperation(op, Box::new(left), Box::new(right))
    }

    fn unop(op: UnaryOperator, expr: Expression) -> Expression {
        Expression::UnaryOperation(op, Box::new(expr))
    }

    fn sym(s: &str) -> Expression {
        Expression::Symbol(s.parse().unwrap())
    }

    #[test]
    fn test_from_str() {
        assert_eq!("34".parse::<Expression>().unwrap(), num(34));
        assert_eq!(
            "34+7".parse::<Expression>().unwrap(),
            binop(BinaryOperator::Plus, num(34), num(7))
        );
        assert_eq!(
            "34+7-4".parse::<Expression>().unwrap(),
            binop(
                BinaryOperator::Minus,
                binop(BinaryOperator::Plus, num(34), num(7)),
                num(4),
            )
        );
        assert_eq!(
            "-1+5*20/6".parse::<Expression>().unwrap(),
            binop(
                BinaryOperator::IntDivide,
                binop(
                    BinaryOperator::Multiply,
                    binop(
                        BinaryOperator::Plus,
                        unop(UnaryOperator::Minus, num(1)),
                        num(5)
                    ),
                    num(20),
                ),
                num(6),
            )
        );
        assert_eq!(
            "X+1".parse::<Expression>().unwrap(),
            binop(BinaryOperator::Plus, sym("X"), num(1))
        );
        assert_eq!(
            "1+LABEL".parse::<Expression>().unwrap(),
            binop(BinaryOperator::Plus, num(1), sym("LABEL"))
        );
        assert_eq!(
            "A+B".parse::<Expression>().unwrap(),
            binop(BinaryOperator::Plus, sym("A"), sym("B"))
        );
        assert_eq!(
            "-X".parse::<Expression>().unwrap(),
            unop(UnaryOperator::Minus, sym("X"))
        );
        assert_eq!(
            "X*Y+Z".parse::<Expression>().unwrap(),
            binop(
                BinaryOperator::Plus,
                binop(BinaryOperator::Multiply, sym("X"), sym("Y")),
                sym("Z"),
            )
        );
        assert_eq!(
            "*+1".parse::<Expression>().unwrap(),
            binop(BinaryOperator::Plus, Expression::Asterisk, num(1))
        );
        assert_eq!(
            "*-5".parse::<Expression>().unwrap(),
            binop(BinaryOperator::Minus, Expression::Asterisk, num(5))
        );
        assert_eq!(
            "LABEL-*".parse::<Expression>().unwrap(),
            binop(BinaryOperator::Minus, sym("LABEL"), Expression::Asterisk)
        );
    }
}
