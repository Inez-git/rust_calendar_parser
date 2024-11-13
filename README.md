# rust_calendar_parser
## Description
A parser built in Rust for parsing Google Calendar events using Pest grammar rules and converting them to JSON format.

## Links
- crate:
- docs:

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
- event: ....
- ......
```
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
- parse <input_file> <output_file> ...
- help ...
- credits ...

## Application
- easily access event fields; 
- analyze your schedule;
- or integrate event data into other apps.
