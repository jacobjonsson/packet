use js_parser::parse;
use js_printer::Printer;
use std::fs;
use std::path::PathBuf;
use std::{env, time::Instant};

struct Arguments {
    input_file: PathBuf,
    out_file: Option<PathBuf>,
}

fn main() {
    let now = Instant::now();
    let input_file = env::args().nth(1).expect("Input file is required");
    let out_file = env::args().nth(2).map(PathBuf::from);
    let args = Arguments {
        input_file: PathBuf::from(&input_file),
        out_file: out_file,
    };
    let content = fs::read_to_string(args.input_file).expect("Failed to read file");

    let ast = parse(&content);
    if let Some(out_file) = args.out_file {
        let output = Printer::new().print_program(&ast);
        fs::write(out_file, output).expect("Failed to write to file");
    }
    println!("Done in {}ms", now.elapsed().as_millis());
}
