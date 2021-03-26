use std::collections::HashMap;

use bundler::Bundler;
use fs::FSMock;

#[test]
fn test_bundler() {
    let mut files: HashMap<String, String> = HashMap::new();
    files.insert("a.js".into(), "import b from \"./b.js\";".into());
    files.insert("b.js".into(), "export default function b() {}".into());
    let fs = FSMock::new(files);
    let mut bundler = Bundler::new(Box::new(fs));
    bundler.scan(vec!["a.js"]);
}
