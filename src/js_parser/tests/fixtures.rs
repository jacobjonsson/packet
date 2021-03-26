use js_lexer::Lexer;
use js_parser::Parser;
use logger::LoggerImpl;

use std::fs;
use std::path::PathBuf;

macro_rules! test_fixture {
    ($name:ident, $file:expr) => {
        #[test]
        fn $name() {
            let file_path = PathBuf::from(format!(
                "{}/tests/fixtures/{}",
                env!("CARGO_MANIFEST_DIR"),
                $file
            ));
            let content = fs::read_to_string(file_path).expect("Failed to read file");

            let logger = LoggerImpl::new();
            let lexer = Lexer::new(&content, &logger);
            let mut parser = Parser::new(lexer, &logger);
            let _ = parser.parse_program();
        }
    };
}

test_fixture!(angular_1_2_5, "angular-1.2.5.js");

test_fixture!(jquery_1_9_1, "jquery-1.9.1.js");
