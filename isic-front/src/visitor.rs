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
    fn visit_expr(&mut self, expr: &Expr);
    fn visit_fn_call(&mut self, call: &FnCall);
    fn visit_assignment(&mut self, assignment: &Assignment);
    fn visit_statement(&mut self, stmt: &Statement);

    fn visit_program(&mut self, program: &IsiProgram) {
        for stmt in &program.statements {
            stmt.visit(self);
        }
    }
}
