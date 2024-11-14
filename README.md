# Rust Calendar Events Parser
## Description
A parser built in Rust for parsing Google Calendar events using Pest grammar rules and converting them to JSON format.

## Links
- crate: https://crates.io/crates/rust_calendar_parser
- docs: https://docs.rs/rust_calendar_parser

## Parsing process 
The parser reads Google Calendar event data, identifies specific fields, and parses each according to predefined Pest grammar rules. Each rule matches specific patterns in the event data, allowing the parser to extract structured information from raw text and organize it into JSON format for easy access.

## Fields
A typical event in Calendar has such fields:<br>
**`DTSTART`** - starting date and time for the event in the format YYYYMMDDTHHMMSSZ. The T separates the date and time, and Z indicates that the time is in UTC (Coordinated Universal Time);<br>
**`DTEND`** - ending date a dn time for the event, in the same format as DTSTART;<br>
**`DTSTAMP`** - date and time the event was created or last modified;<br>
**`UID`** - a globally unique identifier for the event, often used to avoid duplication and syncronize events across different platforms;<br>
**`CREATED`** - the date and time when the event was created;<br>
**`LAST-MODIFIED`** - the date and time when the event was last modified.<br>
**`SEQUENCE`** - a sequence number for the event, typically used to track revisions or updates to an event. For example, 0 means this is the first version of the event;<br>
**`STATUS`** - the status of the event, such as whether it is confirmed, tentative, or cancelled;<br>
**`SUMMARY`** - a short description or tittle of the event, such as the subject or topic;<br>
**`TRANSP`** - specifies the transparency of the event, determining whether the event is shown as "busy" or "free" on a calendar.<br>

## Grammar rules
- new_line: Matches a newline character to separate fields within an event.
- letter_or_digit: Matches any letter (uppercase or lowercase) or digit.
- digit: Matches a single numeric digit (0-9).
- datetime: Matches a date and time in the format YYYYMMDDTHHMMSSZ.
- string_value: A general string without newline characters, allowing specific punctuation.
- email_address: Defines the format of a valid email address for uid.
- transp_value: Specifies transparency options, "TRANSP" or "OPAQUE," to indicate if the event is marked as busy or free.
- status_value: Specifies valid status options for an event: "CONFIRMED," "TENTATIVE," or "CANCELLED".

- event: Defines the structure of a full calendar event, with all required fields in a specific order.
- begin: Matches the start of an event with the exact text "BEGIN".
- dtstart: Specifies the start date and time of the event, formatted as "DTSTART:&lt;datetime&gt;".
- dtend: Specifies the end date and time of the event, formatted as "DTEND:&lt;datetime&gt;".
- dtstamp: The timestamp indicating when the event was created or last modified, formatted as "DTSTAMP:&lt;datetime&gt;".
- uid: Unique identifier for the event, structured as "UID:<email_address>".
- created: The date and time when the event was initially created, formatted as "CREATED:&lt;datetime&gt;".
- last_modified: The date and time when the event was last modified, formatted as "LAST-MODIFIED:&lt;datetime&gt;".
- sequence: A sequence number tracking event updates, represented as "SEQUENCE:<digit(s)>".
- status: The event's status, such as "confirmed," "tentative," or "canceled", formatted as "STATUS:<status_value>".
- summary: A brief title or description of the event, formatted as "SUMMARY:<string_value>".
- transp: Indicates the event's transparency (visibility), formatted as "TRANSP:<transp_value>".
- end: Marks the end of an event with the text "END".
 
```rust
event = ${ begin ~ new_line ~ dtstart ~ new_line ~ dtend ~ new_line ~ dtstamp ~ new_line ~ uid ~ new_line ~ created ~ new_line ~ last_modified ~ new_line ~ sequence ~ new_line ~ status ~ new_line ~ summary ~ new_line ~ transp ~ new_line ~ end }

begin = ${ "BEGIN" ~ ":" ~ "VEVENT"}
dtstart = ${ "DTSTART" ~ ":" ~ datetime }
dtend = ${ "DTEND" ~ ":" ~ datetime }
dtstamp = ${ "DTSTAMP" ~ ":" ~ datetime }
uid = ${ "UID" ~ ":" ~ email_address }
created = ${ "CREATED" ~ ":" ~ datetime }
last_modified = ${ "LAST-MODIFIED" ~ ":" ~ datetime }
sequence = ${ "SEQUENCE" ~ ":" ~ digit+ }
status = ${ "STATUS" ~ ":" ~ status_value }
summary = ${ "SUMMARY" ~ ":" ~ string_value }
transp = ${ "TRANSP" ~ ":" ~ transp_value }
end = ${ "END" ~ ":" ~ "VEVENT"}

status_value = { "CONFIRMED" | "TENTATIVE" | "CANCELLED" }
transp_value = { "TRANSP" | "OPAQUE" }
email_address = { (letter_or_digit | "." | "_")+ ~ "@" ~ letter_or_digit+ ~ "." ~ letter_or_digit+ }
datetime = { digit{8} ~ "T" ~ digit{6} ~ "Z" }
string_value = { !new_line ~ ( '\u{20}'..'\u{21}' | '\u{23}'..'\u{5B}' | '\u{5D}'..'\u{7A}' )* }
digit = { '0'..'9' }
letter_or_digit = { 'a'..'z' | 'A'..'Z' | '0'..'9' }
new_line = { "\n" }
```

## CLI commands
**`parse <input_file> <output_file>`**
Parses a specified input file of Google Calendar events and converts it to JSON format, saving the output to the specified file;<br>
**`help`**
Displays information on all available CLI commands and usage examples;<br>
**`credits`**
Shows author information and contributors to the project;<br>
**`description`**
Provides a brief description of the project, including its purpose and features.

## Application
- easily access event fields; 
- analyze your schedule;
- or integrate event data into other apps.
