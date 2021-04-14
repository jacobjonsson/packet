/// Spans represents a region in source file, used for error reporting.
/// Note that the start is inclusive but the end is not.
#[derive(Debug, Clone, PartialEq)]
pub struct Span {
    pub start: usize,
    pub end: usize,
}

impl Span {
    /// Creates a new span
    pub fn new(start: usize, end: usize) -> Span {
        Span { start, end }
    }
}
