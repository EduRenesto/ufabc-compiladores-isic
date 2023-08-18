use std::collections::HashMap;

use isic_front::{ast::{IsiProgram, Ident, BinaryOp}, span::Span, visitor::{IsiVisitor, Visitable}};

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum IsiType {
    Int,
    String,
    Bool,
    Unit,
}

pub struct SymbolInfo {
    ty: IsiType,
    declaration: Span,
    used: bool,
}

#[derive(Debug)]
pub struct CheckError {
    span: Span,
    desc: String,
}

pub struct TypeCk<'a> {
    program: &'a IsiProgram,
    sym_table: HashMap<Ident, SymbolInfo>,
}

impl<'a> TypeCk<'a> {
    pub fn new(program: &'a IsiProgram) -> TypeCk<'a> {
        TypeCk {
            program,
            sym_table: HashMap::new(),
        }
    }

    pub fn check(&mut self) -> Result<(), CheckError> {
        self.visit_program(self.program)?;

        Ok(())
    }
}

impl<'a> IsiVisitor for TypeCk<'a> {
    type Ret = Result<IsiType, CheckError>;

    fn visit_int_literal(&mut self, _lit: &isic_front::ast::IntLiteral) -> Self::Ret {
        Ok(IsiType::Int)
    }

    fn visit_string_literal(&mut self, _lit: &isic_front::ast::StringLiteral) -> Self::Ret {
        Ok(IsiType::String)
    }

    fn visit_ident(&mut self, id: &Ident) -> Self::Ret {
        match self.sym_table.get(id) {
            Some(ref sym) => Ok(sym.ty),
            None => Err(CheckError {
                span: Span { start: 0, end: 0 },
                desc: format!("Undefined variable {}", id.0),
            })
        }
    }

    fn visit_decl(&mut self, decl: &isic_front::ast::VarDecl) -> Self::Ret {
        let span = Span { start: 0, end: 0 };

        if self.sym_table.contains_key(&decl.var_name) {
            return Err(CheckError {
                span,
                desc: format!("Redeclaration of variable {}", decl.var_name.0),
            });
        }

        let ty = match decl.var_type.0.as_str() {
            "int" => Ok(IsiType::Int),
            "string" => Ok(IsiType::String),
            t@_      => Err(CheckError {
                span,
                desc: format!("Unknown type {}", t),
            }),
        }?;

        self.sym_table.insert(
            decl.var_name.clone(),
            SymbolInfo { ty, declaration: span, used: false }
        );

        Ok(ty)
    }

    fn visit_bin_expr(&mut self, bexpr: &isic_front::ast::BinExpr) -> Self::Ret {
        let left  = self.visit_expr(&bexpr.1)?;
        let right = self.visit_expr(&bexpr.2)?;

        if left != right {
            return Err(CheckError {
                span: Span { start: 0, end: 0 },
                desc: format!("Mismatched types for binary expression: left is {:?}, right is {:?}", left, right),
            });
        }

        match bexpr.0 {
            BinaryOp::Add => { Ok(left) },
            BinaryOp::Sub |
            BinaryOp::Mul |
            BinaryOp::Div |
            BinaryOp::Gt  |
            BinaryOp::Lt  |
            BinaryOp::Geq |
            BinaryOp::Leq => {
                match left {
                    IsiType::String |
                    IsiType::Unit => Err(CheckError {
                        span: Span { start: 0, end: 0 },
                        desc: format!("Operator {:?} is not defined between terms of type {:?}", bexpr.0, left),
                    }),
                    _ => Ok(left)
                }
            },
            BinaryOp::Eq |
            BinaryOp::Neq => { Ok(IsiType::Bool) },
        }
    }

    fn visit_fn_call(&mut self, _call: &isic_front::ast::FnCall) -> Self::Ret {
        Ok(IsiType::Unit)
    }

    fn visit_assignment(&mut self, assignment: &isic_front::ast::Assignment) -> Self::Ret {
        let left = self.visit_ident(&assignment.ident)?;
        let right = self.visit_expr(&assignment.val)?;

        if left != right {
            return Err(CheckError {
                span: Span { start: 0, end: 0 },
                desc: format!("Mismatched types for assignment: tried to assign a {:?} to a {:?}", right, left),
            });
        }

        Ok(left)
    }

    fn visit_program(&mut self, program: &IsiProgram) -> Self::Ret {
        for stmt in &program.statements {
            self.visit_statement(stmt)?;
        }

        Ok(IsiType::Unit)
    }
}
