use crate::ast::*;

#[macro_export]
macro_rules! impl_visitable {
    ($type_name:ident, $fn_name:ident) => {
        impl Visitable for $type_name {
            fn visit<V: IsiVisitor + ?Sized>(&self, visitor: &mut V) {
                visitor.$fn_name (self);
            }
        }
    }
}

pub trait Visitable {
    fn visit<V: IsiVisitor + ?Sized>(&self, visitor: &mut V);
}

pub trait IsiVisitor {
    fn visit_int_literal(&mut self, lit: &IntLiteral);

    fn visit_string_literal(&mut self, lit: &StringLiteral);

    fn visit_ident(&mut self, id: &Ident);

    fn visit_decl(&mut self, decl: &VarDecl);

    fn visit_expr(&mut self, expr: &Expr) {
        match expr {
            Expr::Ident(ident) => self.visit_ident(ident),
            Expr::ImmInt(imm) => self.visit_int_literal(imm),
            Expr::ImmString(imm) => self.visit_string_literal(imm),
            Expr::BinExpr(_, _, _) => todo!(),
            Expr::FnCall(_fcall) => todo!(),
        }
    }

    fn visit_fn_call(&mut self, call: &FnCall);

    fn visit_assignment(&mut self, assignment: &Assignment);

    fn visit_statement(&mut self, stmt: &Statement) {
        match stmt {
            Statement::Assignment(ass) => self.visit_assignment(ass),
            Statement::Decl(decl)      => self.visit_decl(decl),
            Statement::FnCall(call)    => self.visit_fn_call(call),
        }
    }

    fn visit_program(&mut self, program: &IsiProgram) {
        for stmt in &program.statements {
            stmt.visit(self);
        }
    }
}
