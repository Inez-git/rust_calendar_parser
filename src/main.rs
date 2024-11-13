use anyhow::Result;
use rust_calendar_parser::*;
use std::env;
use std::path::Path;

fn main() -> Result<()> {
    let args: Vec<String> = env::args().collect();

    if args.len() < 2 {
        print_help();
        return Ok(());
    }

    match args[1].as_str() {
        "parse" => {
            if args.len() != 4 {
                eprintln!("Error: 'parse' command requires an input file and an output file.");
                return Ok(());
            }
            let input_file_name = &args[2];
            let output_file_name = &args[3];

            let input_path = Path::new(input_file_name);
            let output_path = Path::new(output_file_name);

            match parse_events_from_txt_to_json(input_path, output_path) {
                Ok(_) => println!(
                    "Events have been written to JSON file: {}",
                    output_file_name
                ),
                Err(e) => eprintln!("Error parsing file '{}': {}", input_file_name, e),
            }
        }
        "help" => {
            print_help();
        }
        "credits" => {
            println!("Google Calendar Events Parser by Inna Stetsiuk KN-4 Group 4");
        }
        "description" => {
            println!("A parser built in Rust for parsing and analyzing Google Calendar events using Pest grammar.");
        }
        _ => {
            eprintln!("Unknown command '{}'", args[1]);
            println!("Run 'cargo run help' to see available commands.");
        }
    }
    Ok(())
}

fn print_help() {
    println!("Available commands:");
    println!("  parse <input_file> <output_file> - Parse events and output as JSON");
    println!("  help - Show this help message");
    println!("  credits - Show the program credits");
    println!("  description - Show the program description");
}
