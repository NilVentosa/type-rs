use console::Term;
use glob::glob;
use rand::seq::SliceRandom;
use rand::thread_rng;
use std::env;
use std::{
    fs::File,
    io::{BufRead, BufReader},
};

pub fn run() -> Result<(), String> {
    let path = get_random_file_path().unwrap();
    let line = get_random_line_from_file(&path).unwrap();
    let term = Term::stdout();
    term.write_str(&format!("{}", line)).unwrap();
    let thing = term.read_char().unwrap();
    term.clear_line().unwrap();
    term.write_line(&format!("{}", thing)).unwrap();
    Ok(())
}

fn get_random_file_path() -> Option<String> {
    if let Ok(home) = env::var("HOME") {
        let entries: Vec<String> = glob("{home}/.cargo/registry/src/**/*.rs")
            .unwrap()
            .map(|p| p.unwrap().display().to_string())
            .collect();
        Some(entries.choose(&mut thread_rng()).unwrap().to_string())
    } else {
        None
    }
}

fn get_random_line_from_file(file_path: &str) -> Option<String> {
    if let Ok(file) = File::open(file_path) {
        let lines: Vec<String> = BufReader::new(file)
            .lines()
            .filter_map(|l| l.ok())
            .filter(|l| !l.contains('/'))
            .map(|l| l.trim().to_string())
            .filter(|l| l.len() > 10)
            .collect();
        return Some(lines.choose(&mut thread_rng()).unwrap().to_string());
    }
    None
}
