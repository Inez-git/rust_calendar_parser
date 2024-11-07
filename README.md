# rust_calendar_parser
## A parser built in Rust for parsing and analyzing Google Calendar events using Pest grammar.


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

## Application
- easily access event fields; 
- analyze your schedule;
- or integrate event data into other apps.
