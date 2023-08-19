use std::collections::HashMap;

use isic_front::{
    ast::{Expr, Ident, IsiProgram},
    span::Span,
    visitor::IsiVisitor,
};

use crate::CheckError;

#[derive(Debug)]
struct UsageInfo {
    declared: Span,
    assignments: Vec<Span>,
    uses: Vec<Span>,
}

pub struct UsageCk<'a> {
    program: &'a IsiProgram,
    sym_table: HashMap<Ident, UsageInfo>,
}

impl<'a> UsageCk<'a> {
    pub fn new(program: &'a IsiProgram) -> UsageCk<'a> {
        UsageCk {
            program,
            sym_table: HashMap::new(),
        }
    }

    pub fn check(&mut self) -> Vec<CheckError> {
        self.visit_program(&self.program);

        let mut ret = vec![];

        for (id, info) in self.sym_table.iter() {
            if info.uses.is_empty() {
                ret.push(CheckError {
                    span: info.declared,
                    desc: format!("Variable {} was declared but not used anywhere", id.name),
                });
            } else if info.assignments.is_empty() {
                ret.push(CheckError {
                    span: info.declared,
                    desc: format!("Variable {} is used without being written to", id.name),
                });
            }
        }

        ret
    }

    fn mark_assigment(&mut self, id: &Ident, span: Span) {
        let entry = self.sym_table.get_mut(id).unwrap();

        entry.assignments.push(span);
    }

    fn mark_usage(&mut self, id: &Ident, span: Span) {
        let entry = self.sym_table.get_mut(id).unwrap();

        entry.uses.push(span);
    }
}

impl<'a> IsiVisitor for UsageCk<'a> {
    type Ret = ();

    fn visit_int_literal(&mut self, _lit: &isic_front::ast::IntLiteral) -> Self::Ret {
        // do nothing
    }

    fn visit_float_literal(&mut self, _lit: &isic_front::ast::FloatLiteral) -> Self::Ret {
        // do nothing
    }

    fn visit_string_literal(&mut self, _lit: &isic_front::ast::StringLiteral) -> Self::Ret {
        // do nothing
    }

    fn visit_ident(&mut self, _id: &Ident) -> Self::Ret {
        // do nothing
    }

    fn visit_decl(&mut self, decl: &isic_front::ast::VarDecl) -> Self::Ret {
        if self.sym_table.contains_key(&decl.var_name) {
            return;
        }

        let span = decl.span;

        self.sym_table.insert(
            decl.var_name.clone(),
            UsageInfo {
                declared: span,
                assignments: vec![],
                uses: vec![],
            },
        );
    }

    fn visit_expr(&mut self, expr: &isic_front::ast::Expr) -> Self::Ret {
        let span = expr.get_span();

        match expr {
            Expr::Ident(ident) => self.mark_usage(ident, span),
            Expr::BinExpr(bexp) => self.visit_bin_expr(bexp),
            Expr::FnCall(call) => self.visit_fn_call(call),
            _ => {}
        }
    }

    fn visit_bin_expr(&mut self, bexpr: &isic_front::ast::BinExpr) -> Self::Ret {
        self.visit_expr(&bexpr.1);
        self.visit_expr(&bexpr.2);
    }

    fn visit_fn_call(&mut self, call: &isic_front::ast::FnCall) -> Self::Ret {
        if call.fname.name == "leia" {
            // caso especifico: funcao "leia", que escreve
            // nos args.
            let span = call.get_span();

            if let Expr::Ident(ref id) = call.args[0] {
                self.mark_assigment(id, span);
            }

            return;
        }

        for arg in &call.args {
            self.visit_expr(arg);
        }
    }

    fn visit_assignment(&mut self, assignment: &isic_front::ast::Assignment) -> Self::Ret {
        let span = assignment.get_span();

        self.mark_assigment(&assignment.ident, span);

        self.visit_expr(&assignment.val);
    }

    fn visit_conditional(&mut self, conditional: &isic_front::ast::Conditional) -> Self::Ret {
        self.visit_expr(&conditional.cond);

        for stmt in &conditional.taken {
            self.visit_statement(stmt);
        }

        for stmt in &conditional.not_taken {
            self.visit_statement(stmt);
        }
    }

    fn visit_while_loop(&mut self, while_loop: &isic_front::ast::WhileLoop) -> Self::Ret {
        self.visit_expr(&while_loop.cond);

        for stmt in &while_loop.body {
            self.visit_statement(stmt);
        }
    }

    fn visit_do_while_loop(&mut self, do_while_loop: &isic_front::ast::DoWhileLoop) -> Self::Ret {
        self.visit_expr(&do_while_loop.cond);

        for stmt in &do_while_loop.body {
            self.visit_statement(stmt);
        }
    }
}
