use anyhow::anyhow;
use pest::Parser;
use rust_calendar_parser::*;

#[cfg(test)]
mod rust_calendar_parser_tests {
    use super::*;

    #[test]
    fn test_new_line() -> anyhow::Result<()> {
        let newline_input = "\n";
        let parsed_data = EventParser::parse(Rule::new_line, newline_input)?
            .next()
            .ok_or_else(|| anyhow!("Failed to parse new line"))?;
        assert_eq!(parsed_data.as_str(), newline_input);
        Ok(())
    }

    #[test]
    fn test_letter_or_digit() -> anyhow::Result<()> {
        let correct_letters_and_digits = ["a", "b", "z", "A", "B", "Z", "0", "1", "9"];
        for input in correct_letters_and_digits {
            let parsed_data = EventParser::parse(Rule::letter_or_digit, input)?
                .next()
                .ok_or_else(|| anyhow!("Failed to parse letter_or_digit"))?;
            assert_eq!(parsed_data.as_str(), input);
        }
        Ok(())
    }

    #[test]
    fn test_digit() -> anyhow::Result<()> {
        let correct_digits = ["0", "1", "2", "3", "4", "5", "6", "7", "8", "9"];
        for input in correct_digits {
            let parsed_data = EventParser::parse(Rule::digit, input)?
                .next()
                .ok_or_else(|| anyhow!("Failed to parse digit"))?;
            assert_eq!(parsed_data.as_str(), input);
        }
        Ok(())
    }

    #[test]
    fn test_string_value() -> anyhow::Result<()> {
        let valid_chars = ["h ello*", "wOrl 0d"];
        for input in valid_chars {
            let parsed_data = EventParser::parse(Rule::string_value, input)?
                .next()
                .ok_or_else(|| anyhow!("Failed to parse string"))?;
            assert_eq!(parsed_data.as_str(), input);
        }
        Ok(())
    }

    #[test]
    fn test_datetime() -> anyhow::Result<()> {
        let correct_datetimes = ["20241028T091500Z", "20231201T120000Z"];
        for input in correct_datetimes {
            let parsed_data = EventParser::parse(Rule::datetime, input)?
                .next()
                .ok_or_else(|| anyhow!("Failed to parse datetime"))?;
            assert_eq!(parsed_data.as_str(), input);
        }
        Ok(())
    }

    #[test]
    fn test_email_address() -> anyhow::Result<()> {
        let valid_emails = [
            "test@example.com",
            "user_name@domain.org",
            "john.doe@site.net",
        ];
        for input in valid_emails {
            let parsed_data = EventParser::parse(Rule::email_address, input)?
                .next()
                .ok_or_else(|| anyhow!("Failed to parse email_address"))?;
            assert_eq!(parsed_data.as_str(), input);
        }
        Ok(())
    }

    #[test]
    fn test_transp_value() -> anyhow::Result<()> {
        let valid_transp_values = ["TRANSP", "OPAQUE"];
        for input in valid_transp_values {
            let parsed_data = EventParser::parse(Rule::transp_value, input)?
                .next()
                .ok_or_else(|| anyhow!("Failed to parse transp_value"))?;
            assert_eq!(parsed_data.as_str(), input);
        }
        Ok(())
    }

    #[test]
    fn test_status_value() -> anyhow::Result<()> {
        let valid_status_values = ["CONFIRMED", "TENTATIVE", "CANCELLED"];
        for input in valid_status_values {
            let parsed_data = EventParser::parse(Rule::status_value, input)?
                .next()
                .ok_or_else(|| anyhow!("Failed to parse status_value"))?;
            assert_eq!(parsed_data.as_str(), input);
        }
        Ok(())
    }

    #[test]
    fn test_begin() -> anyhow::Result<()> {
        let input = "BEGIN:VEVENT";
        let parsed_data = EventParser::parse(Rule::begin, input)?
            .next()
            .ok_or_else(|| anyhow!("Failed to parse begin"))?;
        assert_eq!(parsed_data.as_str(), input);
        Ok(())
    }

    #[test]
    fn test_dtstart() -> anyhow::Result<()> {
        let input = "DTSTART:20241028T091500Z";
        let parsed_data = EventParser::parse(Rule::dtstart, input)?
            .next()
            .ok_or_else(|| anyhow!("Failed to parse dtstart"))?;
        assert_eq!(parsed_data.as_str(), input);
        Ok(())
    }

    #[test]
    fn test_dtend() -> anyhow::Result<()> {
        let input = "DTEND:20241028T094000Z";
        let parsed_data = EventParser::parse(Rule::dtend, input)?
            .next()
            .ok_or_else(|| anyhow!("Failed to parse dtend"))?;
        assert_eq!(parsed_data.as_str(), input);
        Ok(())
    }

    #[test]
    fn test_dtstamp() -> anyhow::Result<()> {
        let input = "DTSTAMP:20241107T061409Z";
        let parsed_data = EventParser::parse(Rule::dtstamp, input)?
            .next()
            .ok_or_else(|| anyhow!("Failed to parse dtstamp"))?;
        assert_eq!(parsed_data.as_str(), input);
        Ok(())
    }

    #[test]
    fn test_uid() -> anyhow::Result<()> {
        let input = "UID:abc123@calendar.com";
        let parsed_data = EventParser::parse(Rule::uid, input)?
            .next()
            .ok_or_else(|| anyhow!("Failed to parse uid"))?;
        assert_eq!(parsed_data.as_str(), input);
        Ok(())
    }

    #[test]
    fn test_created() -> anyhow::Result<()> {
        let input = "CREATED:20241028T091121Z";
        let parsed_data = EventParser::parse(Rule::created, input)?
            .next()
            .ok_or_else(|| anyhow!("Failed to parse created"))?;
        assert_eq!(parsed_data.as_str(), input);
        Ok(())
    }

    #[test]
    fn test_last_modified() -> anyhow::Result<()> {
        let input = "LAST-MODIFIED:20241028T091121Z";
        let parsed_data = EventParser::parse(Rule::last_modified, input)?
            .next()
            .ok_or_else(|| anyhow!("Failed to parse last_modified"))?;
        assert_eq!(parsed_data.as_str(), input);
        Ok(())
    }

    #[test]
    fn test_sequence() -> anyhow::Result<()> {
        let input = "SEQUENCE:5";
        let parsed_data = EventParser::parse(Rule::sequence, input)?
            .next()
            .ok_or_else(|| anyhow!("Failed to parse sequence"))?;
        assert_eq!(parsed_data.as_str(), input);
        Ok(())
    }

    #[test]
    fn test_status() -> anyhow::Result<()> {
        let valid_statuses = ["STATUS:CONFIRMED", "STATUS:TENTATIVE", "STATUS:CANCELLED"];
        for input in valid_statuses {
            let parsed_data = EventParser::parse(Rule::status, input)?
                .next()
                .ok_or_else(|| anyhow!("Failed to parse status"))?;
            assert_eq!(parsed_data.as_str(), input);
        }
        Ok(())
    }

    #[test]
    fn test_summary() -> anyhow::Result<()> {
        let valid_summaries = [
            "SUMMARY:Lecture",
            "SUMMARY:Linear Algebra",
            "SUMMARY:Project Review",
        ];

        for input in valid_summaries {
            let parsed_data = EventParser::parse(Rule::summary, input)?
                .next()
                .ok_or_else(|| anyhow!("Failed to parse summary"))?;
            assert_eq!(parsed_data.as_str(), input);
        }
        Ok(())
    }

    #[test]
    fn test_transp() -> anyhow::Result<()> {
        let valid_transp_values = ["TRANSP:OPAQUE", "TRANSP:TRANSP"];
        for input in valid_transp_values {
            let parsed_data = EventParser::parse(Rule::transp, input)?
                .next()
                .ok_or_else(|| anyhow!("Failed to parse transparency"))?;
            assert_eq!(parsed_data.as_str(), input);
        }
        Ok(())
    }

    #[test]
    fn test_end() -> anyhow::Result<()> {
        let input = "END:VEVENT";
        let parsed_data = EventParser::parse(Rule::end, input)?
            .next()
            .ok_or_else(|| anyhow!("Failed to parse end"))?;
        assert_eq!(parsed_data.as_str(), input);
        Ok(())
    }

    #[test]
    fn test_event() -> anyhow::Result<()> {
        let valid_event = "BEGIN:VEVENT\nDTSTART:20241028T091500Z\nDTEND:20241028T094000Z\nDTSTAMP:20241107T061409Z\nUID:1bgs@google.com\nCREATED:20241028T091121Z\nLAST-MODIFIED:20241028T091121Z\nSEQUENCE:0\nSTATUS:CONFIRMED\nSUMMARY:Pilates\nTRANSP:OPAQUE\nEND:VEVENT";
        let parsed_data = EventParser::parse(Rule::event, valid_event)?
            .next()
            .ok_or_else(|| anyhow!("Failed to parse 'event' rule"))?;
        assert_eq!(parsed_data.as_str(), valid_event);
        Ok(())
    }
}
