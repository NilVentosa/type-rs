use console::Term;
use glob::glob;
use rand::seq::SliceRandom;
use rand::thread_rng;
use std::env;
use std::{
    error::Error,
    fs::File,
    io::{BufRead, BufReader},
};

pub fn run() -> Result<(), Box<dyn Error>> {
    let line = get_random_line()?;
    let term = Term::stdout();
    term.write_str(&line)?;
    Ok(())
}

fn get_random_file_path() -> Result<String, Box<dyn Error>> {
    let home = env::var("HOME")?;
    let entries: Vec<String> = glob(&format!("{home}/.cargo/registry/src/**/*.rs"))?
        .map(|p| p.unwrap().display().to_string())
        .collect();
    println!("{}", entries.len());
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
