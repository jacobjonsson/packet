use fs::FS;
use js_ast::AST;
use js_lexer;
use js_parser::Parser;
use logger::LoggerImpl;

#[derive(Debug)]
pub struct File {
    representation: FileRepresentation,
}

#[derive(Debug)]
pub enum FileRepresentation {
    JS(FileRepresentationJS),
}

#[derive(Debug)]
pub struct FileRepresentationJS {
    pub ast: AST,
}

pub struct Bundler {
    fs: Box<dyn FS>,
    // TODO: This queue should be processed in a async manner.
    queue: Vec<String>,
    files: Vec<File>,
}

impl Bundler {
    pub fn new(fs: Box<dyn FS>) -> Bundler {
        Bundler {
            fs: fs,
            queue: Vec::new(),
            files: Vec::new(),
        }
    }

    pub fn scan(&mut self, entry_files: Vec<&str>) {
        for file_path in entry_files {
            self.parse_file(file_path);
        }

        println!("{:?}", self.files);
    }
}

impl Bundler {
    fn parse_file(&mut self, path: &str) {
        let content = match self.fs.read_file(path) {
            Ok(c) => c,
            Err(_) => {
                return;
            }
        };

        let logger = LoggerImpl::new();
        let lexer = js_lexer::create(&content);
        let ast = Parser::new(lexer, &logger).parse_program();

        // TODO: Look at the import records and push them to the queue.

        self.files.push(File {
            representation: FileRepresentation::JS(FileRepresentationJS { ast }),
        });

        // Pop off the next item on the queue and process it.
        let next_file = self.queue.pop();
        if let Some(path) = next_file {
            self.parse_file(&path);
        }
    }
}
