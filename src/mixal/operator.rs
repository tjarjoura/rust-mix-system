use std::str::FromStr;

#[derive(Debug, PartialEq)]
pub enum UnaryOperator {
    Plus,
    Minus,
}

impl UnaryOperator {
    pub fn starts_with(s: &str) -> bool {
        s.starts_with(['+', '-'])
    }
}

impl FromStr for UnaryOperator {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "+" => Ok(UnaryOperator::Plus),
            "-" => Ok(UnaryOperator::Minus),
            _ => anyhow::bail!("Unrecognized unary operator: {}", s),
        }
    }
}

#[derive(Debug, PartialEq)]
pub enum BinaryOperator {
    Plus,
    Minus,
    Multiply,
    IntDivide,
    ScaledDivide,
    Colon,
}

impl BinaryOperator {
    /// Finds the leftmost Binary operator and returns the position and slice containining it
    pub fn find_leftmost_in(s: &str) -> Option<(usize, &str)> {
        let pos = s.find(['+', '-', '*', '/', ':'])?;
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_unop_starts_with() {
        assert_eq!(UnaryOperator::starts_with("+37"), true);
        assert_eq!(UnaryOperator::starts_with("-37"), true);
        assert_eq!(UnaryOperator::starts_with("37"), false);
        assert_eq!(UnaryOperator::starts_with(""), false);
    }

    #[test]
    fn test_unop_from_str() {
        assert_eq!("+".parse::<UnaryOperator>().unwrap(), UnaryOperator::Plus);
        assert_eq!("-".parse::<UnaryOperator>().unwrap(), UnaryOperator::Minus);
        assert!("".parse::<UnaryOperator>().is_err());
        assert!("sdfsd".parse::<UnaryOperator>().is_err());
    }

    #[test]
    fn test_binop_find_leftmost_in() {
        assert_eq!(BinaryOperator::find_leftmost_in("3+4"), Some((1, "+")));
        assert_eq!(BinaryOperator::find_leftmost_in("3"), None);
        assert_eq!(BinaryOperator::find_leftmost_in("3*5+4"), Some((1, "*")));
        assert_eq!(BinaryOperator::find_leftmost_in("37/5+4"), Some((2, "/")));
        assert_eq!(BinaryOperator::find_leftmost_in("37//5+4"), Some((2, "//")));
        assert_eq!(BinaryOperator::find_leftmost_in("102456:9"), Some((6, ":")));
        assert_eq!(BinaryOperator::find_leftmost_in("9-7"), Some((1, "-")));
        assert_eq!(BinaryOperator::find_leftmost_in(""), None);
    }

    #[test]
    fn test_binop_from_str() {
        assert_eq!("+".parse::<BinaryOperator>().unwrap(), BinaryOperator::Plus);
        assert_eq!(
            "-".parse::<BinaryOperator>().unwrap(),
            BinaryOperator::Minus
        );
        assert_eq!(
            "*".parse::<BinaryOperator>().unwrap(),
            BinaryOperator::Multiply
        );
        assert_eq!(
            "/".parse::<BinaryOperator>().unwrap(),
            BinaryOperator::IntDivide
        );
        assert_eq!(
            "//".parse::<BinaryOperator>().unwrap(),
            BinaryOperator::ScaledDivide
        );
        assert_eq!(
            ":".parse::<BinaryOperator>().unwrap(),
            BinaryOperator::Colon
        );

        assert!("".parse::<BinaryOperator>().is_err());
        assert!("a".parse::<BinaryOperator>().is_err());
    }
}
