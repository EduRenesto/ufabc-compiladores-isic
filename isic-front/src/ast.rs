use crate::{visitor::{Visitable, IsiVisitor}, impl_visitable};

#[derive(Debug, PartialEq, Eq)]
pub struct IntLiteral(pub u64);

#[derive(Debug, PartialEq, Eq)]
pub struct StringLiteral(pub String);

#[derive(Clone, Hash, Debug, PartialEq, Eq)]
pub struct Ident(pub String);

#[derive(Debug, PartialEq, Eq)]
pub struct VarDecl {
    pub var_name: Ident,
    pub var_type: Ident,
}

impl VarDecl {
    pub fn new(var_name: Ident, var_type: Ident) -> VarDecl {
        VarDecl {
            var_name,
            var_type,
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
}

#[derive(Debug, PartialEq, Eq)]
pub enum Expr {
    Ident(Ident),
    ImmInt(IntLiteral),
    ImmString(StringLiteral),
    BinExpr(BinaryOp, Box<Expr>, Box<Expr>),
    FnCall(FnCall),
}

impl Expr {
    pub fn get_type(&self) -> Option<Ident> {
        match self {
            Expr::Ident(_) => None,
            Expr::ImmInt(_) => Some(Ident("int".to_string())),
            Expr::ImmString(_) => Some(Ident("string".to_string())),
            Expr::BinExpr(op, _, _) => {
                match op {
                    BinaryOp::Add => Some(Ident("int".to_string())),
                    BinaryOp::Sub => Some(Ident("int".to_string())),
                    BinaryOp::Mul => Some(Ident("int".to_string())),
                    BinaryOp::Div => Some(Ident("int".to_string())),
                    BinaryOp::Gt  => Some(Ident("bool".to_string())),
                    BinaryOp::Lt  => Some(Ident("bool".to_string())),
                    BinaryOp::Geq => Some(Ident("bool".to_string())),
                    BinaryOp::Leq => Some(Ident("bool".to_string())),
                    BinaryOp::Eq  => Some(Ident("bool".to_string())),
                    BinaryOp::Neq => Some(Ident("bool".to_string())),
                }
            },
            Expr::FnCall(_) => None,
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
        FnCall {
            fname,
            args,
        }
    }
}

#[derive(Debug, PartialEq, Eq)]
pub struct Assignment {
    pub ident: Ident,
    pub val: Expr,
}

impl Assignment {
    pub fn new(ident: Ident, val: Expr) -> Assignment {
        Assignment {
            ident,
            val,
        }
    }
}

#[derive(Debug, PartialEq, Eq)]
pub enum Statement {
    Decl(VarDecl),
    FnCall(FnCall),
    Assignment(Assignment),
}

#[derive(Debug, PartialEq, Eq)]
pub struct IsiProgram {
    pub statements: Vec<Statement>,
}

impl IsiProgram {
    pub fn new(statements: Vec<Statement>) -> IsiProgram {
        IsiProgram {
            statements,
        }
    }
}

impl_visitable!(IntLiteral, visit_int_literal);
impl_visitable!(StringLiteral, visit_string_literal);
impl_visitable!(Ident, visit_ident);
impl_visitable!(VarDecl, visit_decl);
impl_visitable!(Expr, visit_expr);
impl_visitable!(FnCall, visit_fn_call);
impl_visitable!(Assignment, visit_assignment);
impl_visitable!(Statement, visit_statement);
