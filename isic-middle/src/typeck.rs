use std::collections::HashMap;

use isic_front::{
    ast::{BinaryOp, Ident, IsiProgram},
    visitor::IsiVisitor,
};

use crate::CheckError;
use crate::IsiType;
use crate::SymbolInfo;

/// O analisador de tipos do isic. Valida uma AST e garante
/// que ela não possua erros de tipos.
pub struct TypeCk<'a> {
    /// Referencia ao programa a ser interpretado.
    program: &'a IsiProgram,
    /// Tabela de tipos das variáveis do programa.
    pub sym_table: HashMap<Ident, SymbolInfo>,
}

impl<'a> TypeCk<'a> {
    /// Cria um novo type checker.
    pub fn new(program: &'a IsiProgram) -> TypeCk<'a> {
        TypeCk {
            program,
            sym_table: HashMap::new(),
        }
    }

    /// Faz a checagem de tipos da AST. Se não houverem erros, retorna Ok(()).
    /// Caso hajam, retorna um vetor com os erros para serem reportados.
    pub fn check(&mut self) -> Result<(), Vec<CheckError>> {
        let prog = self.visit_program(self.program);

        let mut ret = vec![];

        for err in prog.into_iter().filter(|r| r.is_err()) {
            ret.push(err.unwrap_err());
        }

        if ret.is_empty() {
            Ok(())
        } else {
            Err(ret)
        }
    }
}

impl<'a> IsiVisitor for TypeCk<'a> {
    type Ret = Result<IsiType, CheckError>;

    fn visit_int_literal(&mut self, _lit: &isic_front::ast::IntLiteral) -> Self::Ret {
        Ok(IsiType::Int)
    }

    fn visit_float_literal(&mut self, _lit: &isic_front::ast::FloatLiteral) -> Self::Ret {
        Ok(IsiType::Float)
    }

    fn visit_string_literal(&mut self, _lit: &isic_front::ast::StringLiteral) -> Self::Ret {
        Ok(IsiType::String)
    }

    fn visit_ident(&mut self, id: &Ident) -> Self::Ret {
        match self.sym_table.get(id) {
            Some(ref sym) => Ok(sym.ty),
            None => Err(CheckError {
                span: id.span,
                desc: format!("Undefined variable {}", id.name),
            }),
        }
    }

    fn visit_decl(&mut self, decl: &isic_front::ast::VarDecl) -> Self::Ret {
        let span = decl.span;

        if self.sym_table.contains_key(&decl.var_name) {
            return Err(CheckError {
                span,
                desc: format!("Redeclaration of variable {}", decl.var_name.name),
            });
        }

        let ty = match decl.var_type.name.as_str() {
            "int" => Ok(IsiType::Int),
            "float" => Ok(IsiType::Float),
            "string" => Ok(IsiType::String),
            t @ _ => Err(CheckError {
                span,
                desc: format!("Unknown type {}", t),
            }),
        }?;

        self.sym_table.insert(
            decl.var_name.clone(),
            SymbolInfo {
                ty,
                declaration: span,
            },
        );

        Ok(ty)
    }

    fn visit_bin_expr(&mut self, bexpr: &isic_front::ast::BinExpr) -> Self::Ret {
        let span = bexpr.get_span();

        let left = self.visit_expr(&bexpr.1)?;
        let right = self.visit_expr(&bexpr.2)?;

        if left != right {
            return Err(CheckError {
                span,
                desc: format!(
                    "Mismatched types for binary expression: left is {:?}, right is {:?}",
                    left, right
                ),
            });
        }

        match bexpr.0 {
            BinaryOp::Add => Ok(left),
            BinaryOp::Sub | BinaryOp::Mul | BinaryOp::Div => match left {
                IsiType::String | IsiType::Unit => Err(CheckError {
                    span,
                    desc: format!(
                        "Operator {:?} is not defined between terms of type {:?}",
                        bexpr.0, left
                    ),
                }),
                _ => Ok(left),
            },
            BinaryOp::Mod => match left {
                IsiType::Int => Ok(IsiType::Int),
                _ => Err(CheckError {
                    span,
                    desc: format!(
                        "Operator % is only defined between terms of type Int"
                    ),
                }),
            }
            BinaryOp::Gt
            | BinaryOp::Lt
            | BinaryOp::Geq
            | BinaryOp::Leq
            | BinaryOp::Eq
            | BinaryOp::Neq => Ok(IsiType::Bool),
            BinaryOp::And | BinaryOp::Or => match left {
                IsiType::Bool => Ok(IsiType::Bool),
                _ => Err(CheckError {
                    span,
                    desc: format!("Operator {:?} is only defined between terms of type Bool", bexpr.0),
                }),
            }
        }
    }

    fn visit_fn_call(&mut self, _call: &isic_front::ast::FnCall) -> Self::Ret {
        Ok(IsiType::Unit)
    }

    fn visit_assignment(&mut self, assignment: &isic_front::ast::Assignment) -> Self::Ret {
        let span = assignment.get_span();

        let left = self.visit_ident(&assignment.ident)?;
        let right = self.visit_expr(&assignment.val)?;

        if left != right {
            return Err(CheckError {
                span,
                desc: format!(
                    "Mismatched types for assignment: tried to assign a {:?} to a {:?}",
                    right, left
                ),
            });
        }

        Ok(left)
    }

    fn visit_conditional(&mut self, conditional: &isic_front::ast::Conditional) -> Self::Ret {
        let cond_span = conditional.cond.get_span();
        let cond_ty = self.visit_expr(&conditional.cond)?;

        if cond_ty != IsiType::Bool {
            return Err(CheckError {
                span: cond_span,
                desc: format!(
                    "The type of conditionals must be Bool, found {:?} instead",
                    cond_ty
                ),
            });
        }

        for stmt in &conditional.taken {
            self.visit_statement(stmt)?;
        }

        for stmt in &conditional.not_taken {
            self.visit_statement(stmt)?;
        }

        Ok(IsiType::Unit)
    }

    fn visit_while_loop(&mut self, while_loop: &isic_front::ast::WhileLoop) -> Self::Ret {
        let cond_span = while_loop.cond.get_span();
        let cond_ty = self.visit_expr(&while_loop.cond)?;

        if cond_ty != IsiType::Bool {
            return Err(CheckError {
                span: cond_span,
                desc: format!(
                    "The type of conditionals must be Bool, found {:?} instead",
                    cond_ty
                ),
            });
        }

        for stmt in &while_loop.body {
            self.visit_statement(stmt)?;
        }

        Ok(IsiType::Unit)
    }

    fn visit_do_while_loop(&mut self, do_while_loop: &isic_front::ast::DoWhileLoop) -> Self::Ret {
        let cond_span = do_while_loop.cond.get_span();
        let cond_ty = self.visit_expr(&do_while_loop.cond)?;

        if cond_ty != IsiType::Bool {
            return Err(CheckError {
                span: cond_span,
                desc: format!(
                    "The type of conditionals must be Bool, found {:?} instead",
                    cond_ty
                ),
            });
        }

        for stmt in &do_while_loop.body {
            self.visit_statement(stmt)?;
        }

        Ok(IsiType::Unit)
    }

    fn visit_negation(&mut self, neg: &isic_front::ast::Negation) -> Self::Ret {
        let ty = self.visit_expr(&neg.expr)?;

        if ty != IsiType::Bool {
            Err(CheckError {
                span: neg.get_span(),
                desc: format!(
                    "The negation operator can only be applied to terms of type Bool, found {:?} instead",
                    ty,
                )
            })
        } else {
            Ok(IsiType::Bool)
        }
    }

    fn visit_multi_decl(&mut self, decls: &isic_front::ast::MultiVarDecl) -> Self::Ret {
        for decl in &decls.0 {
            self.visit_decl(decl)?;
        }

        Ok(IsiType::Unit)
    }
}
