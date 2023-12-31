use std::{collections::HashMap, io::Write};

use isic_front::{
    ast::{Expr, Ident, IsiProgram},
    visitor::IsiVisitor,
};
use isic_middle::{CheckError, IsiType, SymbolInfo};

/// O emissor de código C da IsiLanguage.
///
/// Ele é implementado como um IsiVisitor, e cada função visitadora
/// escreve na saída o código C equivalente ao nó sendo visitado.
///
/// **Importante.** O emissor **assume que a AST já foi validada,
/// e não há erros de tipos**. Se uma AST inválida for passada para o
/// emissor, o comportamento é não definido. Provavelmente ocorrerá
/// um panic. Portanto, sempre valide a AST antes de usar o emissor.
pub struct CEmitter<'a, W: Write> {
    /// Referencia ao programa a ser interpretado.
    program: &'a IsiProgram,
    /// Tabela de tipos das variáveis do programa.
    sym_table: &'a HashMap<Ident, SymbolInfo>,
    /// Referência a saída onde o código C será escrito.
    output: &'a mut W,
    /// Nível de identação atual do código C.
    id_level: usize,
}

impl<'a, W: Write> CEmitter<'a, W> {
    /// Cria um novo emissor.
    pub fn new(
        program: &'a IsiProgram,
        sym_table: &'a HashMap<Ident, SymbolInfo>,
        output: &'a mut W,
    ) -> CEmitter<'a, W> {
        CEmitter {
            program,
            sym_table,
            output,
            id_level: 4,
        }
    }

    /// Emite o código C do programa associado.
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

    fn pad(&self) -> String {
        " ".repeat(self.id_level)
    }

    fn emit_print(&mut self, call: &isic_front::ast::FnCall) {
        let arg = &call.args[0];

        match arg {
            Expr::Ident(ref ident) => {
                let sym = self.sym_table.get(ident).unwrap();

                let fmt = match sym.ty {
                    IsiType::Int => "%d",
                    IsiType::Float => "%f",
                    IsiType::String => "%s",
                    _ => todo!(),
                };

                writeln!(
                    self.output,
                    "{}printf(\"{}\\n\", {});",
                    self.pad(),
                    fmt,
                    ident.name
                )
                .unwrap();
            }
            Expr::ImmInt(ref imm) => {
                writeln!(self.output, "{}printf(\"%d\\n\", {});", self.pad(), imm.0).unwrap();
            }
            Expr::ImmFloat(ref imm) => {
                writeln!(self.output, "{}printf(\"%f\\n\", {});", self.pad(), imm.0).unwrap();
            }
            Expr::ImmString(ref imm) => {
                writeln!(self.output, "{}printf(\"{}\\n\");", self.pad(), imm.0).unwrap();
            }
            _ => todo!(),
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
                    IsiType::Int => "%d",
                    IsiType::Float => "%f",
                    IsiType::String => "%s",
                    _ => todo!(),
                };

                writeln!(
                    self.output,
                    "{}scanf(\"{}\", &{});",
                    self.pad(),
                    fmt,
                    ident.name
                )
                .unwrap();
            }
            _ => todo!(),
        }
    }
}

impl<'a, W: Write> IsiVisitor for CEmitter<'a, W> {
    type Ret = Result<(), CheckError>;

    fn visit_int_literal(&mut self, lit: &isic_front::ast::IntLiteral) -> Result<(), CheckError> {
        write!(self.output, "{}", lit.0).unwrap();

        Ok(())
    }

    fn visit_float_literal(
        &mut self,
        lit: &isic_front::ast::FloatLiteral,
    ) -> Result<(), CheckError> {
        write!(self.output, "{}f", lit.0).unwrap();

        Ok(())
    }

    fn visit_string_literal(
        &mut self,
        lit: &isic_front::ast::StringLiteral,
    ) -> Result<(), CheckError> {
        write!(self.output, "{}", lit.0).unwrap();

        Ok(())
    }

    fn visit_ident(&mut self, id: &Ident) -> Result<(), CheckError> {
        write!(self.output, "{}", id.name).unwrap();

        Ok(())
    }

    fn visit_decl(&mut self, decl: &isic_front::ast::VarDecl) -> Result<(), CheckError> {
        let ty = match self.sym_table.get(&decl.var_name).unwrap().ty {
            IsiType::Int => "int",
            IsiType::Float => "float",
            IsiType::String => "char*",
            _ => todo!(),
        };

        writeln!(self.output, "{}{} {};", self.pad(), ty, decl.var_name.name).unwrap();

        Ok(())
    }

    fn visit_fn_call(&mut self, call: &isic_front::ast::FnCall) -> Result<(), CheckError> {
        match call.fname.name.as_str() {
            "escreva" => self.emit_print(call),
            "leia" => self.emit_scan(call),
            _ => todo!(),
        };

        Ok(())
    }

    fn visit_assignment(
        &mut self,
        assignment: &isic_front::ast::Assignment,
    ) -> Result<(), CheckError> {
        write!(self.output, "{}{} = ", self.pad(), assignment.ident.name).unwrap();

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
            isic_front::ast::BinaryOp::Mod => "%",
            isic_front::ast::BinaryOp::Gt => ">",
            isic_front::ast::BinaryOp::Lt => "<",
            isic_front::ast::BinaryOp::Geq => ">=",
            isic_front::ast::BinaryOp::Leq => "<=",
            isic_front::ast::BinaryOp::Eq => "==",
            isic_front::ast::BinaryOp::Neq => "!=",
            isic_front::ast::BinaryOp::And => "&&",
            isic_front::ast::BinaryOp::Or => "||",
        };

        write!(self.output, " {} ", op).unwrap();

        self.visit_expr(&bexpr.2)?;

        write!(self.output, ")").unwrap();

        Ok(())
    }

    fn visit_conditional(&mut self, conditional: &isic_front::ast::Conditional) -> Self::Ret {
        write!(self.output, "{}if (", self.pad()).unwrap();

        self.visit_expr(&conditional.cond)?;

        writeln!(self.output, ") {{").unwrap();

        self.id_level += 4;

        for stmt in &conditional.taken {
            self.visit_statement(stmt)?;
        }

        self.id_level -= 4;

        writeln!(self.output, "{}}}", self.pad()).unwrap();

        if !conditional.not_taken.is_empty() {
            writeln!(self.output, "{}else {{", self.pad()).unwrap();

            self.id_level += 4;

            for stmt in &conditional.not_taken {
                self.visit_statement(stmt)?;
            }

            self.id_level -= 4;

            writeln!(self.output, "{}}}", self.pad()).unwrap();
        }

        Ok(())
    }

    fn visit_while_loop(&mut self, while_loop: &isic_front::ast::WhileLoop) -> Self::Ret {
        write!(self.output, "{}while (", self.pad()).unwrap();

        self.visit_expr(&while_loop.cond)?;

        writeln!(self.output, ") {{").unwrap();

        self.id_level += 4;

        for stmt in &while_loop.body {
            self.visit_statement(stmt)?;
        }

        self.id_level -= 4;

        writeln!(self.output, "{}}}", self.pad()).unwrap();

        Ok(())
    }

    fn visit_do_while_loop(&mut self, do_while_loop: &isic_front::ast::DoWhileLoop) -> Self::Ret {
        writeln!(self.output, "{}do {{", self.pad()).unwrap();

        self.id_level += 4;

        for stmt in &do_while_loop.body {
            self.visit_statement(stmt)?;
        }
        self.id_level -= 4;

        write!(self.output, "{}}} while (", self.pad()).unwrap();

        self.visit_expr(&do_while_loop.cond)?;

        writeln!(self.output, ");").unwrap();

        Ok(())
    }

    fn visit_negation(&mut self, neg: &isic_front::ast::Negation) -> Self::Ret {
        write!(self.output, "!(").unwrap();

        self.visit_expr(&neg.expr)?;

        write!(self.output, ")").unwrap();

        Ok(())
    }

    fn visit_multi_decl(&mut self, decls: &isic_front::ast::MultiVarDecl) -> Self::Ret {
        for decl in &decls.0 {
            self.visit_decl(decl)?;
        }

        Ok(())
    }
}
