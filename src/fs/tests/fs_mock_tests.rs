use std::collections::HashMap;

use fs::{FSMock, FS};

#[test]
fn test_fs_mock() {
    let mut files = HashMap::new();
    files.insert("a.js".into(), "function a() {}".into());
    files.insert("b.js".into(), "function b() {}".into());

    let fs = FSMock::new(files);

    assert_eq!(
        fs.read_file("a.js").unwrap(),
        String::from("function a() {}")
    );
    assert_eq!(
        fs.read_file("b.js").unwrap(),
        String::from("function b() {}")
    );
}
