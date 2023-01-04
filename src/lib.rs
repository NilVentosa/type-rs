use console::style;
use console::Key;
use console::Term;
use glob::glob;
use rand::seq::SliceRandom;
use rand::thread_rng;
use std::env;
use std::fmt;
use std::time::SystemTime;
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
    CodeLine::new(get_random_line().unwrap())
        .play(&term)
        .print_result(&term);
    Ok(())
}

struct CodeLine {
    line: String,
    start_time: Option<SystemTime>,
    seconds: f32,
    ok: f32,
    failed: f32,
}

impl CodeLine {
    fn new(line: String) -> Self {
        CodeLine {
            line,
            start_time: None,
            seconds: 0f32,
            ok: 0f32,
            failed: 0f32,
        }
    }

    fn print_result(self, term: &Term) {
        let acc = ((self.ok / (self.ok + self.failed)) * 100f32).round();
        let cps = ((self.ok / self.seconds) * 100f32).round() / 100f32;
        term.write_line(&format!(
            "    {}{} {}{}",
            style(acc).yellow(),
            style("% accuracy").yellow(),
            style(cps).yellow(),
            style(" chars per second").yellow(),
        ))
        .unwrap();
    }

    fn play(mut self, term: &Term) -> Self {
        term.write_str(&self.line).unwrap();
        self.start_time = Some(SystemTime::now());
        let characters: Vec<char> = self.line.chars().collect();
        term.move_cursor_left(characters.len()).unwrap();
        for c in characters {
            loop {
                let input = term.read_key().unwrap();
                if input == Key::Escape {
                    self.seconds = self.start_time.unwrap().elapsed().unwrap().as_secs_f32();
                    return self;
                } else if input == Key::Char(c) {
                    self.ok += 1f32;
                    term.show_cursor().unwrap();
                    term.write_str(&format!("{}", style(c).green().bold()))
                        .unwrap();
                    break;
                } else {
                    self.failed += 1f32;
                    term.hide_cursor().unwrap();
                    term.write_str(&format!("{}", style(c).red().bold()))
                        .unwrap();
                    term.move_cursor_left(1).unwrap();
                }
            }
        }
        self.seconds = self.start_time.unwrap().elapsed().unwrap().as_secs_f32();
        self
    }
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
