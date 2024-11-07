use rust_calendar_parser::*;
use pest::Parser;
use anyhow::anyhow;

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
}
