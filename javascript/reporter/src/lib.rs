struct Message {
    pub text: String,
    pub location: MessageLocation,
}

struct MessageLocation {
    pub line: usize,
    pub column: usize,
    pub length: usize,
    pub line_text: String,
}

pub fn report_unexpected_token(source: &str, message: &str, start: usize, end: usize) {
    let (line_count, column_count, line_start, line_end) = compute_line_and_column(source, start);

    report_message(Message {
        text: message.into(),
        location: MessageLocation {
            column: column_count,
            line: line_count,
            length: end - start,
            line_text: source[line_start..line_end].into(),
        },
    });
}

fn report_message(message: Message) {
    println!(
        ">[{}:{}] \x1b[0;1;31mError\x1b[0m: \x1b[0;1m{}\x1b[0m\n  {} | {}\n",
        message.location.line,
        message.location.column,
        message.text,
        message.location.line,
        message.location.line_text
    );

    panic!();
}

fn compute_line_and_column(content: &str, offset: usize) -> (usize, usize, usize, usize) {
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
