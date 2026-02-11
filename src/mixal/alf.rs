/// Pseudo-operation in MIXAL that assembles raw characters (text)
#[derive(Debug, PartialEq)]
pub struct Alf {
    pub chars: [char; 5],
}

impl Alf {
    /// Constructs an Alf pseudo-op from a string containing the "character data"
    /// to assemble.
    /// Should begin with one or two blank spaces, and then exactly 5 characters.
    /// If one blank space -- then the first character must be non blank
    fn from_char_data(s: &str) -> anyhow::Result<Self> {
        let char_data = if s.starts_with("  ") {
            &s[2..]
        } else if s.starts_with(" ") {
            &s[1..]
        } else {
            anyhow::bail!(
                "ALF pseudo-op character data must have exactly one or two leading blank spaces"
            );
        };

        if char_data.len() != 5 {
            anyhow::bail!("ALF pseudo-op must be given exactly 5 characters")
        }

        if !char_data.chars().all(|c| Alf::is_valid_mix_character(c)) {
            anyhow::bail!("Invalid character in {}", char_data)
        }

        Ok(Self {
            chars: char_data
                .chars()
                .collect::<Vec<_>>()
                .try_into()
                .map_err(|_| anyhow::anyhow!("Expected exactly 5 characters"))?,
        })
    }

    /// Checks if c is one of the 60 valid characters listed in the reference at the back of the book
    fn is_valid_mix_character(c: char) -> bool {
        c.is_ascii_uppercase() || c.is_ascii_digit() || " .,()+-*/=$<>@;:'".contains(c)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_two_leading_spaces() {
        let result = Alf::from_char_data("  HELLO");
        assert!(result.is_ok());
        let alf = result.unwrap();
        assert_eq!(alf.chars, ['H', 'E', 'L', 'L', 'O']);
    }

    #[test]
    fn test_two_leading_spaces_with_space_in_data() {
        let result = Alf::from_char_data("  HE LO");
        assert!(result.is_ok());
        let alf = result.unwrap();
        assert_eq!(alf.chars, ['H', 'E', ' ', 'L', 'O']);
    }

    #[test]
    fn test_one_leading_space_non_blank_first() {
        let result = Alf::from_char_data(" WORLD");
        assert!(result.is_ok());
        let alf = result.unwrap();
        assert_eq!(alf.chars, ['W', 'O', 'R', 'L', 'D']);
    }

    #[test]
    fn test_digits_and_punctuation() {
        let result = Alf::from_char_data("  123+5");
        assert!(result.is_ok());
        let alf = result.unwrap();
        assert_eq!(alf.chars, ['1', '2', '3', '+', '5']);
    }

    #[test]
    fn test_all_punctuation() {
        let result = Alf::from_char_data("  .,();");
        assert!(result.is_ok());
        let alf = result.unwrap();
        assert_eq!(alf.chars, ['.', ',', '(', ')', ';']);
    }

    #[test]
    fn test_no_leading_spaces() {
        let result = Alf::from_char_data("HELLO");
        assert!(result.is_err());
        assert!(
            result
                .unwrap_err()
                .to_string()
                .contains("one or two leading blank spaces")
        );
    }

    #[test]
    fn test_three_spaces_valid() {
        // Two leading spaces + space as first char + 4 more chars
        let result = Alf::from_char_data("   HELL");
        assert!(result.is_ok());
        let alf = result.unwrap();
        assert_eq!(alf.chars, [' ', 'H', 'E', 'L', 'L']);
    }

    #[test]
    fn test_too_few_characters() {
        let result = Alf::from_char_data("  HELL");
        assert!(result.is_err());
        assert!(
            result
                .unwrap_err()
                .to_string()
                .contains("exactly 5 characters")
        );
    }

    #[test]
    fn test_too_many_characters() {
        let result = Alf::from_char_data("  HELLOO");
        assert!(result.is_err());
        assert!(
            result
                .unwrap_err()
                .to_string()
                .contains("exactly 5 characters")
        );
    }

    #[test]
    fn test_invalid_character_lowercase() {
        let result = Alf::from_char_data("  HeLLO");
        assert!(result.is_err());
        assert!(
            result
                .unwrap_err()
                .to_string()
                .contains("Invalid character")
        );
    }

    #[test]
    fn test_invalid_character_special() {
        let result = Alf::from_char_data("  HEL#O");
        assert!(result.is_err());
        assert!(
            result
                .unwrap_err()
                .to_string()
                .contains("Invalid character")
        );
    }

    #[test]
    fn test_empty_after_spaces() {
        let result = Alf::from_char_data("  ");
        assert!(result.is_err());
        assert!(
            result
                .unwrap_err()
                .to_string()
                .contains("exactly 5 characters")
        );
    }
}
