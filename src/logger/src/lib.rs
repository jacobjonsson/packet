use std::sync::{Arc, Mutex};

#[derive(Clone)]
pub struct Range {
    pub start: usize,
    pub end: usize,
}

#[derive(Clone)]
pub struct MessageLocation {
    pub line: usize,
    pub column: usize,
    pub length: usize,
    pub line_text: String,
}

#[derive(Clone)]
pub enum MessageKind {
    Warning,
    Error,
}

#[derive(Clone)]
pub struct Message {
    pub text: String,
    pub kind: MessageKind,
    pub location: MessageLocation,
}

impl ToString for Message {
    fn to_string(&self) -> String {
        format!(
            ">[{}:{}] \x1b[0;1;31mError\x1b[0m: \x1b[0;1m{}\x1b[0m\n  {} | {}\n",
            self.location.line,
            self.location.column,
            self.text,
            self.location.line,
            self.location.line_text
        )
    }
}

pub enum LoggerLevel {
    Info,
    Warning,
    Error,
}

pub fn compute_line_and_column(content: &str, offset: usize) -> (usize, usize, usize, usize) {
    let mut line_count: usize = 0;
    let mut line_start: usize = 0;
    let mut line_end: usize = content.len();

    let mut prev_char: Option<char> = None;

    // Count lines until the offset
    for (idx, char) in &mut content[..offset].chars().enumerate() {
        match char {
            '\n' => {
                line_start = idx + 1;
                if prev_char != Some('\r') {
                    line_count += 1;
                }
            }
            '\r' => {
                line_start = idx + 1;
                line_count += 1;
            }
            _ => {}
        }

        prev_char = Some(char);
    }

    // Scan until the end of the line
    for (idx, character) in &mut content[offset..].chars().enumerate() {
        if character == '\n' || character == '\r' {
            line_end = offset + idx;
            break;
        }
    }

    (line_count, offset - line_start, line_start, line_end)
}

fn location_data(source: &str, range: Range) -> MessageLocation {
    let (line_count, column_count, line_start, line_end) =
        compute_line_and_column(&source, range.start);

    MessageLocation {
        column: column_count,
        line: line_count,
        length: range.end - range.start,
        line_text: source[line_start..line_end].into(),
    }
}

pub trait Logger {
    fn add_message(&self, message: Message);
    fn has_errors(&self) -> bool;
    fn has_warnings(&self) -> bool;
    fn flush(&self);
    fn add_error(&self, source: &str, range: Range, text: String);
}

pub struct LoggerImpl {
    errors: Arc<Mutex<i64>>,
    warnings: Arc<Mutex<i64>>,
    messages: Arc<Mutex<Vec<Message>>>,
}

impl LoggerImpl {
    pub fn new() -> LoggerImpl {
        LoggerImpl {
            errors: Arc::new(Mutex::new(0)),
            warnings: Arc::new(Mutex::new(0)),
            messages: Arc::new(Mutex::new(Vec::new())),
        }
    }
}

impl Logger for LoggerImpl {
    fn add_message(&self, message: Message) {
        let mut messages = self.messages.lock().unwrap();
        match message.kind {
            MessageKind::Error => eprintln!("{}", message.to_string()),
            MessageKind::Warning => println!("{}", message.to_string()),
        }
        messages.push(message);
    }

    fn has_errors(&self) -> bool {
        if let Ok(errors) = self.errors.lock() {
            return errors.gt(&0);
        } else {
            return false;
        }
    }

    fn has_warnings(&self) -> bool {
        if let Ok(warnings) = self.warnings.lock() {
            return warnings.gt(&0);
        } else {
            return false;
        }
    }

    fn flush(&self) {
        todo!()
    }

    fn add_error(&self, source: &str, range: Range, text: String) {
        self.add_message(Message {
            kind: MessageKind::Error,
            text: text,
            location: location_data(source, range),
        });
    }
}
