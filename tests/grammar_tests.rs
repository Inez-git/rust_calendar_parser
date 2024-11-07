use anyhow::anyhow;
use pest::Parser;
use rust_calendar_parser::*;

#[cfg(test)]
mod rust_calendar_parser_tests {
    use super::*;

    #[test]
    fn test_digit() -> anyhow::Result<()> {
        let correct_digits = ["0", "1", "2", "3", "4", "5", "6", "7", "8", "9"];
        for input in correct_digits {
            let parsed_data = Grammar::parse(Rule::digit, input)?
                .next()
                .ok_or_else(|| anyhow!("Failed to parse digit"))?;
            assert_eq!(parsed_data.as_str(), input);
        }
        Ok(())
    }

    #[test]
    fn test_datetime() -> anyhow::Result<()> {
        let correct_datetimes = ["20241028T091500Z", "20231201T120000Z"];
        for input in correct_datetimes {
            let parsed_data = Grammar::parse(Rule::datetime, input)?
                .next()
                .ok_or_else(|| anyhow!("Failed to parse datetime"))?;
            assert_eq!(parsed_data.as_str(), input);
        }
        Ok(())
    }

    #[test]
    fn test_newline() -> anyhow::Result<()> {
        let newline_input = "\n";
        let parsed_data = Grammar::parse(Rule::newline, newline_input)?
            .next()
            .ok_or_else(|| anyhow!("Failed to parse newline"))?;
        assert_eq!(parsed_data.as_str(), newline_input);
        Ok(())
    }

    #[test]
    fn test_letter_or_digit() -> anyhow::Result<()> {
        let correct_letters_and_digits = ["a", "b", "z", "A", "B", "Z", "0", "1", "9"];
        for input in correct_letters_and_digits {
            let parsed_data = Grammar::parse(Rule::letter_or_digit, input)?
                .next()
                .ok_or_else(|| anyhow!("Failed to parse letter_or_digit"))?;
            assert_eq!(parsed_data.as_str(), input);
        }
        Ok(())
    }

    #[test]
    fn test_string_in_quotes() -> anyhow::Result<()> {
        let valid_chars = ["\"hello\"", "\"world\""];
        for input in valid_chars {
            let parsed_data = Grammar::parse(Rule::string_in_quotes, input)?
                .next()
                .ok_or_else(|| anyhow!("Failed to parse string_in_quotes"))?;
            assert_eq!(parsed_data.as_str(), input);
        }
        Ok(())
    }
}
