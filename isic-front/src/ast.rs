use crate::{visitor::{Visitable, IsiVisitor}, impl_visitable, span::Span};

pub struct Spanned<T: std::fmt::Debug + PartialEq + Eq> {
    pub span: Span,
    pub node: T,
}

#[derive(Debug, PartialEq, Eq)]
pub struct IntLiteral(pub u64);

#[derive(Debug, PartialEq)]
pub struct FloatLiteral(pub f32);

impl std::cmp::Eq for FloatLiteral {} // cheat...

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
pub struct BinExpr(pub BinaryOp, pub Box<Expr>, pub Box<Expr>);

#[derive(Debug, PartialEq, Eq)]
pub enum Expr {
    Ident(Ident),
    ImmInt(IntLiteral),
    ImmFloat(FloatLiteral),
    ImmString(StringLiteral),
    BinExpr(BinExpr),
    FnCall(FnCall),
}

impl Expr {
    pub fn get_type(&self) -> Option<Ident> {
        match self {
            Expr::Ident(_) => None,
            Expr::ImmInt(_) => Some(Ident("int".to_string())),
            Expr::ImmString(_) => Some(Ident("string".to_string())),
            Expr::ImmFloat(_) => Some(Ident("float".to_string())),
            Expr::BinExpr(BinExpr( _, lhs, rhs )) => {
                let lhs_ty = dbg!( lhs.get_type() );
                let rhs_ty = dbg!( rhs.get_type() );

                if lhs_ty == rhs_ty {
                    lhs_ty
                } else {
                    None
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
        IsiProgram {
            statements,
        }
    }
}

impl_visitable!(IntLiteral, visit_int_literal);
impl_visitable!(FloatLiteral, visit_float_literal);
impl_visitable!(StringLiteral, visit_string_literal);
impl_visitable!(Ident, visit_ident);
impl_visitable!(VarDecl, visit_decl);
impl_visitable!(Expr, visit_expr);
impl_visitable!(FnCall, visit_fn_call);
impl_visitable!(Assignment, visit_assignment);
impl_visitable!(Conditional, visit_conditional);
impl_visitable!(WhileLoop, visit_while_loop);
impl_visitable!(DoWhileLoop, visit_do_while_loop);
impl_visitable!(Statement, visit_statement);
impl_visitable!(BinExpr, visit_bin_expr);
