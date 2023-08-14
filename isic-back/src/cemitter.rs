use std::{collections::HashMap, io::Write};

use isic_front::{
    ast::{Ident, IsiProgram, Expr},
    visitor::IsiVisitor, span::Span,
};

use crate::{isi_error::IsiError, symbol::Symbol, builtins::BuiltinType};

pub struct CEmitter<'a, W: Write> {
    program: &'a IsiProgram,
    sym_table: HashMap<Ident, Symbol>,
    errors: Vec<IsiError>,
    output: &'a mut W,
}

impl<'a, W: Write> CEmitter<'a, W> {
    pub fn new(program: &'a IsiProgram, output: &'a mut W) -> CEmitter<'a, W> {
        CEmitter {
            program,
            sym_table: HashMap::new(),
            errors: vec![],
            output,
        }
    }

    pub fn emit(mut self) -> Result<(), Vec<IsiError>> {
        self.write_headers();

        self.visit_program(self.program);

        self.write_footers();

        if self.errors.is_empty() {
            Ok(())
        } else {
            Err(self.errors)
        }
    }

    fn write_headers(&mut self) {
        writeln!(self.output, "/* !!! auto-gerado por isic-back !!! */").unwrap();
        writeln!(self.output, "#include <stdio.h>").unwrap();
        writeln!(self.output, "#include <stdlib.h>").unwrap();
        writeln!(self.output, "").unwrap();
        writeln!(self.output, "int main() {{").unwrap();
    }

    fn write_footers(&mut self) {
        writeln!(self.output, "}}").unwrap();
    }
}

impl<'a, W: Write> IsiVisitor for CEmitter<'a, W> {
    fn visit_int_literal(&mut self, lit: &isic_front::ast::IntLiteral) {
        write!(self.output, "{}", lit.0).unwrap();
        return;
    }

    fn visit_string_literal(&mut self, lit: &isic_front::ast::StringLiteral) {
        write!(self.output, "{}", lit.0).unwrap();
        return;
    }

    fn visit_ident(&mut self, id: &Ident) {
        write!(self.output, "{}", id.0).unwrap();
    }

    fn visit_decl(&mut self, decl: &isic_front::ast::VarDecl) {
        self.sym_table.insert(decl.var_name.clone(), Symbol { ty: decl.var_type.clone() });

        let ty = match decl.var_type.0.as_str() {
            "int"    => Some(BuiltinType::Int),
            "string" => Some(BuiltinType::String),
            _        => None,
        };

        if ty.is_none() {
            self
                .errors
                .push(IsiError {
                    span: Span { start: 0, end: 0 },
                    msg: format!("Unknown type for variable {}: {}", decl.var_name.0, decl.var_type.0),
                });

            return;
        }

        let ty = ty.unwrap();

        let ty = match ty {
            BuiltinType::Int    => "int",
            BuiltinType::String => "char*",
        };

        writeln!(self.output, "    {} {};", ty, decl.var_name.0).unwrap();
    }

    fn visit_fn_call(&mut self, call: &isic_front::ast::FnCall) {
        return;
    }

    fn visit_assignment(&mut self, assignment: &isic_front::ast::Assignment) {
        let sym = self.sym_table.get(&assignment.ident);

        if sym.is_none() {
            self.errors.push(IsiError {
                span: Span { start: 0, end: 0 },
                msg: format!("Assignment to undeclared variable {}", assignment.ident.0),
            });

            return;
        }

        let sym = sym.unwrap();

        let rhs_ty = match assignment.val.get_type() {
            Some(rhs_ty) => Some(rhs_ty),
            None => {
                match assignment.val {
                    Expr::Ident(ref rhs) => {
                        match self.sym_table.get(rhs) {
                            Some(rhs_sym) => {
                                Some(rhs_sym.ty.clone())
                            },
                            None => {
                                self.errors.push(IsiError {
                                    span: Span { start: 0, end: 0 },
                                    msg: format!("Use of undeclared variable {} in assignment to {}", rhs.0, assignment.ident.0),
                                });

                                None
                            }
                        }
                    }
                    _ => todo!()
                }
            },
        };

        if rhs_ty.is_none() {
            return;
        }

        let rhs_ty = rhs_ty.unwrap();

        if sym.ty != rhs_ty {
            self.errors.push(IsiError {
                span: Span { start: 0, end: 0 },
                msg: format!("Type mismatch when trying to assign {} to {}", rhs_ty.0, sym.ty.0),
            });

            return;
        }

        write!(self.output, "    {} = ", assignment.ident.0).unwrap();

        self.visit_expr(&assignment.val);

        writeln!(self.output, ";").unwrap();
    }
}
