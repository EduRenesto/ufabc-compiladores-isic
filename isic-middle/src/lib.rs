use isic_front::span::Span;

pub mod typeck;
pub mod usageck;

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum IsiType {
    Int,
    Float,
    String,
    Bool,
    Unit,
}

pub struct SymbolInfo {
    pub ty: IsiType,
    pub declaration: Span,
}

#[derive(Debug)]
pub struct CheckError {
    pub span: Span,
    pub desc: String,
}
