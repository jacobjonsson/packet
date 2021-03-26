pub struct Source<'a> {
    /// This is the absolute filepath to the source file.
    pub absolute_path: &'a str,

    /// This is the path that should be used in logging messages.
    /// It is relative to the current working directory.
    pub pretty_path: &'a str,

    /// The content of the source file
    pub content: &'a str,
}
