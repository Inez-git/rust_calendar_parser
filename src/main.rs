use pest::Parser;
use rust_calendar_parser::*;

fn main() -> anyhow::Result<()> {
    let input = "BEGIN:VEVENT\nDTSTART:20241028T091500Z\nDTEND:20241028T094000Z\nDTSTAMP:20241107T061409Z\nUID:0vgghd89dnkdfk9fjfbnmae1@google.com\nCREATED:20241028T091121Z\nLAST-MODIFIED:20241028T091121Z\nSEQUENCE:0\nSTATUS:CONFIRMED\nSUMMARY:English\nTRANSP:OPAQUE\nEND:VEVENT";
    match Grammar::parse(Rule::event, input) {
        Ok(parsed) => {
            println!("{:?}", parsed);
        }
        Err(err) => eprintln!("Parsing error: {:?}", err),
    }
    Ok(())
}
