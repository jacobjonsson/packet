use javascript_lexer::Lexer;
use javascript_parser::Parser;
use javascript_printer::Printer;
use logger::LoggerImpl;
use std::env;
use std::fs;
use std::path::PathBuf;

struct Arguments {
    input_file: PathBuf,
    out_file: PathBuf,
}

fn main() {
    let input_file = env::args().nth(1).expect("Input file is required");
    let out_file = env::args().nth(2).expect("Out file is required");
    let args = Arguments {
        input_file: PathBuf::from(input_file),
        out_file: PathBuf::from(out_file),
    };
    let content = fs::read_to_string(args.input_file).expect("Failed to read file");
    let logger = LoggerImpl::new();
    let lexer = Lexer::new(&content, &logger);
    let mut parser = Parser::new(lexer, &logger);
    let program = parser.parse_program();
    let output = Printer::new().print_program(&program);
    fs::write(args.out_file, output).expect("Failed to write to file");
}
