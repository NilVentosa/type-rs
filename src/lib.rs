use code_lines::{get_random_line, Language::Rust, LineConfig};
use console::style;
use console::Key;
use console::Term;
use std::error::Error;
use std::time::SystemTime;

pub fn run() -> Result<(), Box<dyn Error>> {
    let term = Term::stdout();
    term.write_line(&format!(
        "{}",
        style("***press esc key to exit").color256(8)
    ))?;
    let mut code_lines: Vec<CodeLine> = vec![];
    loop {
        let code_line =
            CodeLine::new(get_random_line(&LineConfig { language: Rust }).unwrap()).play(&term);
        code_lines.push(code_line);

        if code_lines.last().is_some() && !code_lines.last().unwrap().completed {
            break;
        }
    }
    print_total_results(&code_lines, &term);
    Ok(())
}

struct CodeLine {
    line: String,
    start_time: Option<SystemTime>,
    seconds: f32,
    ok: f32,
    failed: f32,
    completed: bool,
}

impl CodeLine {
    fn new(line: String) -> Self {
        CodeLine {
            line,
            start_time: None,
            seconds: 0f32,
            ok: 0f32,
            failed: 0f32,
            completed: false,
        }
    }

    fn get_result_string(&self) -> String {
        format!(
            "{}% acc {} cps",
            if (self.ok / (self.ok + self.failed)).is_nan() {
                0f32
            } else {
                ((self.ok / (self.ok + self.failed)) * 100f32).round()
            },
            ((self.ok / self.seconds) * 1000f32).round() / 1000f32
        )
    }

    fn print_result(&self, term: &Term) {
        term.write_line(&format!("    {}", style(self.get_result_string()).yellow(),))
            .unwrap();
    }

    fn play(mut self, term: &Term) -> Self {
        term.write_str(&self.line).unwrap();
        self.start_time = Some(SystemTime::now());
        let characters: Vec<char> = self.line.chars().collect();
        term.move_cursor_left(characters.len()).unwrap();
        'outer: for c in characters {
            loop {
                let input = term.read_key().unwrap();
                if input == Key::Escape {
                    self.seconds = self.start_time.unwrap().elapsed().unwrap().as_secs_f32();
                    term.move_cursor_right(self.line.len() - self.ok as usize)
                        .unwrap();
                    break 'outer;
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
        if self.ok == self.line.len() as f32 {
            self.completed = true;
        }
        self.seconds = self.start_time.unwrap().elapsed().unwrap().as_secs_f32();
        self.print_result(term);
        self
    }
}

fn print_total_results(code_lines: &Vec<CodeLine>, term: &Term) {
    let mut total_time = 0f32;
    let mut total_ok = 0f32;
    let mut total_fail = 0f32;

    for code_line in code_lines {
        total_time += code_line.seconds;
        total_ok += code_line.ok;
        total_fail += code_line.failed;
    }

    term.write_line("").unwrap();
    term.write_line(&format!(
        " {}",
        style(
            CodeLine {
                ok: total_ok,
                failed: total_fail,
                seconds: total_time,
                completed: true,
                start_time: None,
                line: String::from(""),
            }
            .get_result_string()
        )
        .yellow()
    ))
    .unwrap();
}
