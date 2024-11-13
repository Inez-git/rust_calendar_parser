use pest::Parser;
use pest_derive::Parser;
use serde_json::json;
use std::collections::HashMap;
use std::fs::File;
use std::io::{self, BufRead, Write};
use std::path::Path;
use thiserror::Error;

#[derive(Parser)]
#[grammar = "./grammar.pest"]
pub struct EventParser;

#[derive(Error, Debug)]
pub enum ParseError {
    #[error("Parsing error: {0}")]
    ParsingError(String),
    #[error("File error: {0}")]
    FileError(#[from] io::Error),
    #[error("JSON serialization error: {0}")]
    JsonError(#[from] serde_json::Error),
}

pub fn parse_event(input: &str) -> Result<HashMap<String, String>, ParseError> {
    let mut event_map: HashMap<String, String> = HashMap::new();
    let pairs = EventParser::parse(Rule::event, input)
        .map_err(|e| ParseError::ParsingError(format!("{:?}", e)))?;

    for pair in pairs {
        if pair.as_rule() == Rule::event {
            for inner_pair in pair.into_inner() {
                match inner_pair.as_rule() {
                    Rule::dtstart => {
                        let dtstart = inner_pair.into_inner().next().unwrap().as_str();
                        event_map.insert("DTSTART".to_string(), dtstart.to_string());
                    }
                    Rule::dtend => {
                        let dtend = inner_pair.into_inner().next().unwrap().as_str();
                        event_map.insert("DTEND".to_string(), dtend.to_string());
                    }
                    Rule::dtstamp => {
                        let dtstamp = inner_pair.into_inner().next().unwrap().as_str();
                        event_map.insert("DTSTAMP".to_string(), dtstamp.to_string());
                    }
                    Rule::uid => {
                        let uid = inner_pair.into_inner().next().unwrap().as_str();
                        event_map.insert("UID".to_string(), uid.to_string());
                    }
                    Rule::created => {
                        let created = inner_pair.into_inner().next().unwrap().as_str();
                        event_map.insert("CREATED".to_string(), created.to_string());
                    }
                    Rule::last_modified => {
                        let last_modified = inner_pair.into_inner().next().unwrap().as_str();
                        event_map.insert("LAST_MODIFIED".to_string(), last_modified.to_string());
                    }
                    Rule::sequence => {
                        let sequence = inner_pair.into_inner().next().unwrap().as_str();
                        event_map.insert("SEQUENCE".to_string(), sequence.to_string());
                    }
                    Rule::status => {
                        let status = inner_pair.into_inner().next().unwrap().as_str();
                        event_map.insert("STATUS".to_string(), status.to_string());
                    }
                    Rule::summary => {
                        let summary = inner_pair.into_inner().next().unwrap().as_str();
                        event_map.insert("SUMMARY".to_string(), summary.to_string());
                    }
                    Rule::transp => {
                        let transp = inner_pair.into_inner().next().unwrap().as_str();
                        event_map.insert("TRANSP".to_string(), transp.to_string());
                    }
                    _ => {}
                }
            }
        }
    }
    if event_map.is_empty() {
        return Err(ParseError::ParsingError("Empty event".to_string()));
    }
    Ok(event_map)
}

pub fn parse_events_from_txt_to_json(
    input_path: &Path,
    output_path: &Path,
) -> Result<(), ParseError> {
    let file = File::open(input_path)?;
    let reader = io::BufReader::new(file);

    let mut events = Vec::new();
    let mut current_event = String::new();
    let mut inside_event = false;

    for line in reader.lines() {
        let line = line?;

        if line.trim() == "BEGIN:VEVENT" {
            inside_event = true;
            current_event.clear();
            current_event.push_str(&line);
            current_event.push('\n');
        } else if line.trim() == "END:VEVENT" {
            current_event.push_str(&line);
            inside_event = false;

            match parse_event(&current_event) {
                Ok(event) => events.push(event),
                Err(e) => eprintln!("Error parsing event: {}", e),
            }
        } else if inside_event {
            current_event.push_str(&line);
            current_event.push('\n');
        }
    }

    let json_data = json!(events);
    let pretty_json = serde_json::to_string_pretty(&json_data)?;

    let mut output_file = File::create(output_path)?;
    write!(output_file, "{}", pretty_json)?;

    Ok(())
}
