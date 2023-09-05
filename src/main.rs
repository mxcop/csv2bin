use std::{error::Error, io::{self, Write}, process};
use csv::ReaderBuilder;
use is_terminal::IsTerminal;

/// Format :
/// [Values] are converted to bytes [u8].
/// [Newlines] are represented using the number 255 (0xFF).
///! Warn, any non-numbers are converted to 0.

fn main() {
    // Make sure an input stream is specified.
    if atty::is(atty::Stream::Stdin) {
        println!("Missing input stream.");
        println!("Syntax: \"csv2bin < in.csv > out.bin\"");
        return;
    }

    // Make sure an output stream is specified.
    if io::stdout().is_terminal() {
        println!("Missing output stream.");
        println!("Syntax: \"csv2bin < in.csv > out.bin\"");
        return;
    }

    if let Err(err) = parse() {
        println!("error parsing: {}", err);
        process::exit(1);
    }
}

fn parse() -> Result<(), Box<dyn Error>> {
    // Parse the csv input file.
    let mut rdr = ReaderBuilder::new().has_headers(false).from_reader(io::stdin());

    // Convert the csv file into binary.
    let mut buf: Vec<u8> = Vec::new();
    for result in rdr.records() {
        let record = result?;
        for num in record.iter() {
            let num = num.parse::<i16>().unwrap_or(0);
            buf.push(num.max(0).min(254) as u8);
        }
        buf.push(255u8/* Newline */);
    }

    // Write the binary data to stdout.
    io::stdout().write_all(&buf).unwrap();
    Ok(())
}
