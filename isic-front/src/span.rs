#[derive(Debug, Copy, Clone, PartialEq, Eq, Default, Hash)]
/// A estrutura Span representa uma localização no código fonte.
pub struct Span {
    pub start: usize,
    pub end: usize,
}

impl Span {
    pub fn merge(&self, other: &Span) -> Span {
        Span {
            start: self.start.min(other.start),
            end: self.end.max(other.end),
        }
    }
}
