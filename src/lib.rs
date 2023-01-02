use std::{error::Error, io::{BufRead, BufReader}, fs::File};
use console::Term;
use std::io;

pub fn run() -> Result<(), Box<dyn Error>> {
    let term = Term::stdout();
    term.write_str("Hello World!")?;
    let thing = term.read_char().unwrap();
    term.clear_line()?;
    term.write_line(&format!("{}", thing))?;
    Ok(())
}

fn open(filename: &str) -> Result<Box<dyn BufRead>, Box<dyn Error>> {
    match filename {
        "-" => Ok(Box::new(BufReader::new(io::stdin()))),
        _ => Ok(Box::new(BufReader::new(File::open(filename)?))),
    }
}

