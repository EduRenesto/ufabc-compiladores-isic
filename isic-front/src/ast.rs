#[derive(Debug, PartialEq, Eq)]
pub struct IntLiteral(pub u64);

#[derive(Debug, PartialEq, Eq)]
pub struct Ident(pub String);

#[derive(Debug, PartialEq, Eq)]
pub struct StringLiteral(pub String);
