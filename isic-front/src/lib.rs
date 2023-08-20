//! # isic-front
//!
//! Esta crate contém o parser da IsiLang e a AST gerada por ele.
//!
//! Nenhuma regra semântica é implementada nesta crate. O único propósito dela é
//! exportar a AST, parseada a partir do código fonte.

pub mod ast;
pub mod parser;
pub mod span;
pub mod visitor;

pub use peg;
