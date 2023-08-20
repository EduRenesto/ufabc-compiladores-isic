use std::hash::Hash;

use crate::{
    impl_visitable,
    span::Span,
    visitor::{IsiVisitor, Visitable},
};

pub struct Spanned<T: std::fmt::Debug + PartialEq + Eq> {
    pub span: Span,
    pub node: T,
}

#[derive(Debug, PartialEq, Eq)]
pub struct IntLiteral(pub u64, pub Span);

#[derive(Debug, PartialEq)]
pub struct FloatLiteral(pub f32, pub Span);

impl std::cmp::Eq for FloatLiteral {} // cheat...

#[derive(Debug, PartialEq, Eq)]
pub struct StringLiteral(pub String, pub Span);

#[derive(Clone, Debug, Eq)]
pub struct Ident {
    pub name: String,
    pub span: Span,
}

impl std::cmp::PartialEq for Ident {
    fn eq(&self, other: &Self) -> bool {
        self.name == other.name
    }
}

impl Hash for Ident {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        // vamos pular o span
        self.name.hash(state);
    }
}

impl Ident {
    pub fn new(name: &str, span: Span) -> Ident {
        Ident {
            name: name.to_string(),
            span,
        }
    }
}

#[derive(Debug, PartialEq, Eq)]
pub struct VarDecl {
    pub var_name: Ident,
    pub var_type: Ident,
    pub span: Span,
}

impl VarDecl {
    pub fn new(var_name: Ident, var_type: Ident, span: Span) -> VarDecl {
        VarDecl {
            var_name,
            var_type,
            span,
        }
    }
}

#[derive(Debug, PartialEq, Eq)]
pub enum BinaryOp {
    Add,
    Sub,
    Mul,
    Div,
    Gt,
    Lt,
    Geq,
    Leq,
    Eq,
    Neq,
    And,
    Or,
    Mod,
}

#[derive(Debug, PartialEq, Eq)]
pub struct BinExpr(pub BinaryOp, pub Box<Expr>, pub Box<Expr>);

impl BinExpr {
    pub fn get_span(&self) -> Span {
        let lhs = self.1.get_span();
        let rhs = self.2.get_span();

        lhs.merge(&rhs)
    }
}

#[derive(Debug, PartialEq, Eq)]
pub struct Negation {
    pub expr: Box<Expr>,
    pub span: Span,
}

impl Negation {
    pub fn new(expr: Box<Expr>, span: Span) -> Negation {
        Negation {
            expr,
            span,
        }
    }

    pub fn get_span(&self) -> Span {
        self.span.merge(&self.expr.get_span())
    }
}

#[derive(Debug, PartialEq, Eq)]
pub enum Expr {
    Ident(Ident),
    ImmInt(IntLiteral),
    ImmFloat(FloatLiteral),
    ImmString(StringLiteral),
    BinExpr(BinExpr),
    FnCall(FnCall),
    Negation(Negation),
}

impl Expr {
    pub fn get_span(&self) -> Span {
        match self {
            Expr::Ident(ref id) => id.span,
            Expr::ImmInt(ref imm) => imm.1,
            Expr::ImmString(ref imm) => imm.1,
            Expr::ImmFloat(ref imm) => imm.1,
            Expr::BinExpr(ref bexpr) => bexpr.get_span(),
            Expr::FnCall(ref fcall) => fcall.get_span(),
            Expr::Negation(ref neg) => neg.get_span(),
        }
    }
}

#[derive(Debug, PartialEq, Eq)]
pub struct FnCall {
    pub fname: Ident,
    pub args: Vec<Expr>,
}

impl FnCall {
    pub fn new(fname: Ident, args: Vec<Expr>) -> FnCall {
        FnCall { fname, args }
    }

    pub fn get_span(&self) -> Span {
        let mut s = self.fname.span;

        for arg in &self.args {
            s = s.merge(&arg.get_span());
        }

        s
    }
}

#[derive(Debug, PartialEq, Eq)]
pub struct Assignment {
    pub ident: Ident,
    pub val: Expr,
}

impl Assignment {
    pub fn new(ident: Ident, val: Expr) -> Assignment {
        Assignment { ident, val }
    }

    pub fn get_span(&self) -> Span {
        self.ident.span.merge(&self.val.get_span())
    }
}

#[derive(Debug, PartialEq, Eq)]
pub struct Conditional {
    pub cond: Expr,
    pub taken: Vec<Statement>,
    pub not_taken: Vec<Statement>,
}

#[derive(Debug, PartialEq, Eq)]
pub struct WhileLoop {
    pub cond: Expr,
    pub body: Vec<Statement>,
}

#[derive(Debug, PartialEq, Eq)]
pub struct DoWhileLoop {
    pub cond: Expr,
    pub body: Vec<Statement>,
}

#[derive(Debug, PartialEq, Eq)]
pub enum Statement {
    Decl(VarDecl),
    FnCall(FnCall),
    Assignment(Assignment),
    Conditional(Conditional),
    WhileLoop(WhileLoop),
    DoWhileLoop(DoWhileLoop),
}

#[derive(Debug, PartialEq, Eq)]
pub struct IsiProgram {
    pub statements: Vec<Statement>,
}

impl IsiProgram {
    pub fn new(statements: Vec<Statement>) -> IsiProgram {
        IsiProgram { statements }
    }
}

impl_visitable!(IntLiteral, visit_int_literal);
impl_visitable!(FloatLiteral, visit_float_literal);
impl_visitable!(StringLiteral, visit_string_literal);
impl_visitable!(Ident, visit_ident);
impl_visitable!(VarDecl, visit_decl);
impl_visitable!(Expr, visit_expr);
impl_visitable!(FnCall, visit_fn_call);
impl_visitable!(Negation, visit_negation);
impl_visitable!(Assignment, visit_assignment);
impl_visitable!(Conditional, visit_conditional);
impl_visitable!(WhileLoop, visit_while_loop);
impl_visitable!(DoWhileLoop, visit_do_while_loop);
impl_visitable!(Statement, visit_statement);
impl_visitable!(BinExpr, visit_bin_expr);
