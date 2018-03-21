#![allow(unused_must_use)]

use term;

pub struct Print {
    prev_process: Option<String>,
}

impl Print {
    pub fn new() -> Self {
        Print { prev_process: None }
    }

    pub fn info(&mut self, info: &str) {
        self.done_prev_process();

        let mut t = term::stdout().unwrap();
        t.fg(term::color::WHITE);
        writeln!(t, "    {}", info);

        self.prev_process = None;
    }

    pub fn process(&mut self, info: &str) {
        let mut t = term::stdout().unwrap();

        self.done_prev_process();

        t.fg(term::color::WHITE);
        write!(t, "     [");
        t.fg(term::color::BRIGHT_BLUE);
        write!(t, "Process");
        t.fg(term::color::WHITE);
        write!(t, "] ");
        writeln!(t, "{}", info);

        self.prev_process = Some(info.to_string());
    }

    pub fn error(&mut self, err: &str) {
        let mut t = term::stdout().unwrap();

        if let Some(prev_process) = &self.prev_process {
            t.cursor_up();
            t.carriage_return();
            t.delete_line();
            t.fg(term::color::WHITE);
            write!(t, "     [");
            t.fg(term::color::BRIGHT_RED);
            write!(t, "Error");
            t.fg(term::color::WHITE);
            write!(t, "] ");
            writeln!(t, "{} - {}", prev_process, err);
        }

        self.prev_process = None;
    }

    pub fn done_prev_process(&mut self) {
        if let Some(prev_process) = &self.prev_process {
            let mut t = term::stdout().unwrap();
            t.cursor_up();
            t.carriage_return();
            t.delete_line();
            t.fg(term::color::WHITE);
            write!(t, "     [");
            t.fg(term::color::GREEN);
            write!(t, "Done");
            t.fg(term::color::WHITE);
            write!(t, "] ");
            writeln!(t, "{}", prev_process);
        }
    }
}
