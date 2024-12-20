use std::collections::HashMap;
use std::sync::Mutex;
use std::fmt::{self, Write};
use std::io::{self, Write as IoWrite};
use termcolor::{Color, ColorSpec, StandardStream, WriteColor};
use crate::logging::{CommonLogger, Level};

pub struct ConsoleLogger {
    common_logger: CommonLogger,
    mutex: Mutex<()>,
}

impl ConsoleLogger {
    pub fn new(level: Level) -> Self {
        ConsoleLogger {
            common_logger: CommonLogger::new(level),
            mutex: Mutex::new(()),
        }
    }

    pub fn do_log_string(&self, message: &str) {
        let _lock = self.mutex.lock().unwrap();
        
        let mut reading_text = true;
        let mut changed_color = false;
        let mut color = String::new();

        let color_mapping: HashMap<&str, Color> = [
            ("BLUE", Color::Blue),
            ("GREEN", Color::Green),
            ("RED", Color::Red),
            ("YELLOW", Color::Yellow),
            ("WHITE", Color::White),
            ("CYAN", Color::Cyan),
            ("MAGENTA", Color::Magenta),
            ("BRIGHT_BLUE", Color::BrightBlue),
            ("BRIGHT_GREEN", Color::BrightGreen),
            ("BRIGHT_RED", Color::BrightRed),
            ("BRIGHT_YELLOW", Color::BrightYellow),
            ("BRIGHT_WHITE", Color::BrightWhite),
            ("BRIGHT_CYAN", Color::BrightCyan),
            ("BRIGHT_MAGENTA", Color::BrightMagenta),
            ("DEFAULT", Color::Reset),
        ]
        .iter()
        .cloned()
        .collect();

        let mut stdout = StandardStream::stdout(termcolor::ColorChoice::Auto);

        for char_pos in message.chars() {
            if char_pos == '\x1b' { 
                reading_text = !reading_text;
                color.push(char_pos);

                if reading_text {
                    if let Some(&color_code) = color_mapping.get(color.as_str()) {
                        let mut color_spec = ColorSpec::new();
                        color_spec.set_fg(Some(color_code));
                        stdout.set_color(&color_spec).unwrap();
                        changed_color = true;
                    } else {
                        let mut color_spec = ColorSpec::new();
                        color_spec.set_fg(Some(Color::Reset));
                        stdout.set_color(&color_spec).unwrap();
                    }

                    color.clear();
                }
            } else if reading_text {
                write!(stdout, "{}", char_pos).unwrap();
            } else {
                color.push(char_pos);
            }
        }

        if changed_color {
            let mut color_spec = ColorSpec::new();
            color_spec.set_fg(Some(Color::Reset));
            stdout.set_color(&color_spec).unwrap();
        }

        stdout.flush().unwrap();
    }
}

mod logging {
    use std::sync::Arc;
    use std::sync::RwLock;

    #[derive(Debug, PartialEq, Clone, Copy)]
    pub enum Level {
        Debug,
        Info,
        Warn,
        Error,
        Fatal,
    }

    pub struct CommonLogger {
        log_level: Level,
    }

    impl CommonLogger {
        pub fn new(level: Level) -> Self {
            CommonLogger { log_level: level }
        }

        pub fn log(&self, message: &str) {
            println!("{}", message);
        }
    }
}

// Must be added to cargo.toml
[dependencies]
termcolor = "1.1"
