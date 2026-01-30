use std::str::FromStr;

#[derive(Debug, PartialEq)]
pub struct Number(pub u32);

impl FromStr for Number {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if s.len() > 10 {
            anyhow::bail!(
                "Number too big: {}. Numbers cannot be larger than 10 digits",
                s
            )
        }

        Ok(Number(s.parse()?))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_from_str() {
        assert_eq!("12345".parse::<Number>().unwrap(), Number(12345));
        assert_eq!("12".parse::<Number>().unwrap(), Number(12));
        assert_eq!("1234567890".parse::<Number>().unwrap(), Number(1234567890));

        assert!("-12422".parse::<Number>().is_err());
        assert!("7H".parse::<Number>().is_err());
        assert!("".parse::<Number>().is_err());
        assert!(
            "12345678900".parse::<Number>().is_err(),
            "10 digit limit should be respected"
        );
    }
}
