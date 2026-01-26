use std::{fmt::Binary, str::FromStr};

use super::symbol::Symbol;
enum UnaryOperator {
    Plus,
    Minus,
}

enum BinaryOperator {
    Plus,
    Minus,
    Multiply,
    IntDivide,
    ScaledDivide,
    Colon,
}

impl BinaryOperator {
    /// Finds the leftmost Binary operator and returns the position and slice containining it
    fn find_leftmost_in(s: &str) -> Option<(usize, &str)> {
        let pos = s.find('+' | '-' | '*' | '/' | ':')?;
        // first check for the double slash operator, since it has two characters we need to
        // special case it
        if let Some("//") = s.get(pos..pos + 2) {
            Some((pos, &s[pos..pos + 2]))
        // otherwise we know it's one character
        } else {
            Some((pos, &s[pos..pos + 1]))
        }
    }
}

impl FromStr for BinaryOperator {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "+" => Ok(BinaryOperator::Plus),
            "-" => Ok(BinaryOperator::Minus),
            "*" => Ok(BinaryOperator::Multiply),
            "/" => Ok(BinaryOperator::IntDivide),
            "//" => Ok(BinaryOperator::ScaledDivide),
            ":" => Ok(BinaryOperator::Colon),
            _ => anyhow::bail!("Unrecognized binary operator: {}", s),
        }
    }
}

enum Expression {
    Asterisk,
    Symbol(Symbol),
    Number,
    BinaryOperation(BinaryOperator, Box<Expression>, Box<Expression>),
    UnaryOperation(UnaryOperator, Box<Expression>),
}

impl FromStr for Expression {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if s.starts_with('-') {
            return Ok(Expression::UnaryOperation(
                UnaryOperator::Minus,
                Box::new(s[1..].parse()?),
            ));
        } else if s.starts_with('+') {
            return Ok(Expression::UnaryOperation(
                UnaryOperator::Plus,
                Box::new(s[1..].parse()?),
            ));
        } else if let Some((pos, op)) = BinaryOperator::find_leftmost_in(s) {
            return Ok(Expression::BinaryOperation(
                s[pos..pos + op.len()].parse()?,
                Box::new(s[0..pos].parse()?),
                Box::new(s[pos + 1..].parse()?),
            ));
        } else {
            todo!();
        }
    }
}
