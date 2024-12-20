use std::collections::HashSet;
use std::fmt::{self, Write};
use chrono::{Local, Timelike, Date, Datelike};

#[derive(PartialEq, Ord, PartialOrd, Eq, Copy, Clone)]
pub enum Level {
    Debug,
    Info,
    Warn,
    Error,
    Fatal,
}

impl Level {
    pub const LEVEL_NAMES: [&'static str; 5] = ["DEBUG", "INFO", "WARN", "ERROR", "FATAL"];
}

pub struct CommonLogger {
    log_level: Level,
    pattern: String,
    disabled_categories: HashSet<String>,
}

impl CommonLogger {
    pub fn new(level: Level) -> Self {
        Self {
            log_level: level,
            pattern: "%D %T %L [%C] ".to_string(),
            disabled_categories: HashSet::new(),
        }
    }

    pub fn set_pattern(&mut self, pattern: &str) {
        self.pattern = pattern.to_string();
    }

    pub fn enable_category(&mut self, category: &str) {
        self.disabled_categories.remove(category);
    }

    pub fn disable_category(&mut self, category: &str) {
        self.disabled_categories.insert(category.to_string());
    }

    pub fn set_max_level(&mut self, level: Level) {
        self.log_level = level;
    }

    pub fn log(&self, category: &str, level: Level, time: chrono::NaiveDateTime, body: &str) {
        if level <= self.log_level && !self.disabled_categories.contains(category) {
            let mut body2 = body.to_string();

            if !self.pattern.is_empty() {
                let insert_pos = if body2.starts_with("\u{1b}") {
                    body2.find("\u{1b}").unwrap_or(body2.len())
                } else {
                    0
                };

                body2.insert_str(insert_pos, &self.format_pattern(category, level, time));
            }

            self.do_log_string(&body2);
        }
    }

    fn format_pattern(&self, category: &str, level: Level, time: chrono::NaiveDateTime) -> String {
        let mut s = String::new();

        let pattern = self.pattern.as_str();
        let mut pattern_chars = pattern.chars().peekable();
        
        while let Some(c) = pattern_chars.next() {
            if c == '%' {
                if let Some(next_char) = pattern_chars.next() {
                    match next_char {
                        'C' => s.push_str(category),
                        'D' => s.push_str(&format!("{}", time.date())),
                        'T' => s.push_str(&format!("{}", time.time())),
                        'L' => s.push_str(&format!("{:<7}", Level::LEVEL_NAMES[level as usize])),
                        _ => s.push(next_char),
                    }
                }
            } else {
                s.push(c);
            }
        }

        s
    }

    fn do_log_string(&self, message: &str) {
        println!("{}", message);
    }
}

fn main() {
    // Example Usage
    let mut logger = CommonLogger::new(Level::Debug);
    
    let time = Local::now().naive_local();
    logger.log("main", Level::Info, time, "This is a log message.");
}
