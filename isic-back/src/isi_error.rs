use isic_front::span::Span;

#[derive(Debug)]
pub struct IsiError {
    pub span: Span,
    pub msg: String,
}
