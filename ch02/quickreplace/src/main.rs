use text_colorizer::*;

#[derive(Debug)]
struct Arguments {
    target: String,
    replacement: String,
    input_filename: String,
    output_filename: String,
}

fn print_usage() {
    eprintln!(
        "{} - change occurrences of one string into another",
        "quickreplace".green()
    );

    eprintln!("Usage: quickreplace <target> <replacement> <INPUT> <OUTPUT>")
}

fn parse_args() -> Arguments {
    use std::env::args;
    use std::process::exit;

    let args: Vec<String> = args().skip(1).collect();

    if args.len() != 4 {
        print_usage();
        eprintln!(
            "{} wrong number of arguments: expected 4, got {}.",
            "Error:".red().bold(),
            args.len()
        );
        exit(1);
    }

    Arguments {
        target: args[0].clone(),
        replacement: args[1].clone(),
        input_filename: args[2].clone(),
        output_filename: args[3].clone(),
    }
}

use regex::Regex;

fn replace(target: &str, replacement: &str, text: &str) -> Result<String, regex::Error> {
    let regex = Regex::new(target)?;
    Ok(regex.replace_all(text, replacement).to_string())
}

fn main() {
    use std::fs;
    use std::process::exit;

    let args = parse_args();

    let data: String = match fs::read_to_string(&args.input_filename) {
        Ok(v) => v,
        Err(e) => {
            eprintln!(
                "{} failed to read from file '{}': {:?}",
                "Error:".red().bold(),
                args.input_filename,
                e
            );
            exit(1);
        }
    };

    let replaced_data: String = match replace(&args.target, &args.replacement, &data) {
        Ok(v) => v,
        Err(e) => {
            eprintln!("{} failed to replace text: {:?}", "Error:".red().bold(), e);
            exit(1);
        }
    };

    match fs::write(&args.output_filename, &replaced_data) {
        Ok(_) => {}
        Err(e) => {
            eprintln!(
                "{} failed to write to file '{}': {:?}",
                "Error:".red().bold(),
                args.output_filename,
                e
            );
            exit(1);
        }
    }
}
