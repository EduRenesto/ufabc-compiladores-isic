use std::{collections::HashMap, io::Write};

use isic_front::{
    ast::{Ident, IsiProgram, Expr},
    visitor::IsiVisitor, span::Span,
};
use isic_middle::typeck::{SymbolInfo, CheckError, IsiType};

use crate::{isi_error::IsiError, symbol::Symbol, builtins::BuiltinType};

pub struct CEmitter<'a, W: Write> {
    program: &'a IsiProgram,
    sym_table: &'a HashMap<Ident, SymbolInfo>,
    output: &'a mut W,
}

impl<'a, W: Write> CEmitter<'a, W> {
    pub fn new(
        program: &'a IsiProgram,
        sym_table: &'a HashMap<Ident, SymbolInfo>,
        output: &'a mut W
    ) -> CEmitter<'a, W> {
        CEmitter {
            program,
            sym_table,
            output,
        }
    }

    pub fn emit(mut self) -> Result<(), CheckError> {
        self.write_headers();

        self.visit_program(self.program);

        self.write_footers();

        Ok(())
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

    fn emit_print(&mut self, call: &isic_front::ast::FnCall) {
        // TODO(edu): handle the case where the argument
        // is not an ident or an immediate.

        let arg = &call.args[0];

        match arg {
            Expr::Ident(ref ident) => {
                let sym = self.sym_table.get(ident).unwrap();

                let fmt = match sym.ty {
                    IsiType::Int    => "%d",
                    IsiType::String => "%s",
                    _               => todo!(),
                };

                writeln!(self.output, "    printf(\"{}\\n\", {});", fmt, ident.0).unwrap();
            },
            Expr::ImmInt(ref imm) => {
                writeln!(self.output, "    printf(\"%d\\n\", {});", imm.0).unwrap();
            },
            Expr::ImmString(ref imm) => {
                writeln!(self.output, "    printf(\"{}\\n\");", imm.0).unwrap();
            },
            _ => todo!()
        }
    }

    fn emit_scan(&mut self, call: &isic_front::ast::FnCall) {
        // TODO(edu): C shenanigans, precisamos mallocar e depois
        // freear no caso de ser uma string...

        let arg = &call.args[0];

        match arg {
            Expr::Ident(ref ident) => {
                let sym = self.sym_table.get(ident).unwrap();

                let fmt = match sym.ty {
                    IsiType::Int    => "%d",
                    IsiType::String => "%s",
                    _               => todo!(),
                };

                writeln!(self.output, "    scanf(\"{}\\n\", &{});", fmt, ident.0).unwrap();
            },
            _ => todo!()
        }
    }
}

impl<'a, W: Write> IsiVisitor for CEmitter<'a, W> {
    type Ret = Result<(), CheckError>;

    fn visit_int_literal(&mut self, lit: &isic_front::ast::IntLiteral) -> Result<(), CheckError> {
        write!(self.output, "{}", lit.0).unwrap();

        Ok(())
    }

    fn visit_string_literal(&mut self, lit: &isic_front::ast::StringLiteral) -> Result<(), CheckError> {
        write!(self.output, "{}", lit.0).unwrap();

        Ok(())
    }

    fn visit_ident(&mut self, id: &Ident) -> Result<(), CheckError> {
        write!(self.output, "{}", id.0).unwrap();

        Ok(())
    }

    fn visit_decl(&mut self, decl: &isic_front::ast::VarDecl) -> Result<(), CheckError> {
        let ty = match self.sym_table.get(&decl.var_name).unwrap().ty {
            IsiType::Int    => "int",
            IsiType::String => "char*",
            _               => todo!(),
        };

        writeln!(self.output, "    {} {};", ty, decl.var_name.0).unwrap();

        Ok(())
    }

    fn visit_fn_call(&mut self, call: &isic_front::ast::FnCall) -> Result<(), CheckError> {
        match call.fname.0.as_str() {
            "escreva" => self.emit_print(call),
            "leia"    => self.emit_scan(call),
            _         => todo!(),
        };

        Ok(())
    }

    fn visit_assignment(&mut self, assignment: &isic_front::ast::Assignment) -> Result<(), CheckError> {
        write!(self.output, "    {} = ", assignment.ident.0).unwrap();

        self.visit_expr(&assignment.val)?;

        writeln!(self.output, ";").unwrap();

        Ok(())
    }

    fn visit_bin_expr(&mut self, bexpr: &isic_front::ast::BinExpr) -> Result<(), CheckError> {
        write!(self.output, "(").unwrap();

        self.visit_expr(&bexpr.1)?;

        let op = match bexpr.0 {
            isic_front::ast::BinaryOp::Add => "+",
            isic_front::ast::BinaryOp::Sub => "-",
            isic_front::ast::BinaryOp::Mul => "*",
            isic_front::ast::BinaryOp::Div => "/",
            isic_front::ast::BinaryOp::Gt  => ">",
            isic_front::ast::BinaryOp::Lt  => "<",
            isic_front::ast::BinaryOp::Geq => ">=",
            isic_front::ast::BinaryOp::Leq => "<=",
            isic_front::ast::BinaryOp::Eq  => "==",
            isic_front::ast::BinaryOp::Neq => "!=",
        };

        write!(self.output, " {} ", op).unwrap();

        self.visit_expr(&bexpr.2)?;

        write!(self.output, ")").unwrap();

        Ok(())
    }
}
