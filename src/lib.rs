use console::style;
use console::Key;
use console::Term;
use glob::glob;
use rand::seq::SliceRandom;
use rand::thread_rng;
use std::env;
use std::fmt;
use std::{
    error::Error,
    fs::File,
    io::{BufRead, BufReader},
};

#[derive(Debug)]
struct Errrr(String);

impl fmt::Display for Errrr {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "There is an error: {}", self.0)
    }
}

impl Error for Errrr {}

pub fn run() -> Result<(), Box<dyn Error>> {
    let term = Term::stdout();
    term.clear_screen()?;
    line_loop(&term)?;
    Ok(())
}

fn character_loop(term: &Term, c: char) -> Result<(), Box<dyn Error>> {
    loop {
        let input = term.read_key()?;
        if input == Key::Escape {
            return Err(Box::new(Errrr("Oops".into())));
        } else if input == Key::Char(c) {
            term.show_cursor()?;
            term.write_str(&format!("{}", style(c).green().bold()))?;
            break;
        } else {
            term.hide_cursor()?;
            term.write_str(&format!("{}", style(c).red().bold()))?;
            term.move_cursor_left(1)?;
        }
    }
    Ok(())
}

fn line_loop(term: &Term) -> Result<(), Box<dyn Error>> {
    'outer: loop {
        let line = get_random_line()?;
        term.write_str(&line)?;
        let characters: Vec<char> = line.chars().collect();
        term.move_cursor_left(characters.len())?;
        for c in characters {
            if let Err(_) = character_loop(&term, c) {
                break 'outer;
            }
        }
        term.write_line("")?;
    }
    Ok(())
}

fn get_random_file_path() -> Result<String, Box<dyn Error>> {
    let home = env::var("HOME")?;
    let entries: Vec<String> = glob(&format!("{home}/.cargo/registry/src/**/*.rs"))?
        .map(|p| p.unwrap().display().to_string())
        .collect();
    return Ok(entries.choose(&mut thread_rng()).unwrap().to_string());
}

fn get_random_line() -> Result<String, Box<dyn Error>> {
    let path = get_random_file_path()?;
    let file = File::open(path)?;
    let lines: Vec<String> = BufReader::new(file)
        .lines()
        .filter_map(|l| l.ok())
        .filter(|l| !l.contains('/'))
        .map(|l| l.trim().to_string())
        .filter(|l| l.len() > 10)
        .collect();
    Ok(lines.choose(&mut thread_rng()).unwrap().to_string())
}
