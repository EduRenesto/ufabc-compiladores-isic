use crate::ast::*;

#[macro_export]
macro_rules! impl_visitable {
    ($type_name:ident, $fn_name:ident) => {
        impl Visitable for $type_name {
            fn visit<V: IsiVisitor + ?Sized>(&self, visitor: &mut V) {
                visitor.$fn_name(self);
            }
        }
    };
}

pub trait Visitable {
    fn visit<V: IsiVisitor + ?Sized>(&self, visitor: &mut V);
}

/// A trait IsiVisitor é uma das partes mais importantes do projeto. Todas as
/// etapas do compilador são escritas como implementações diferentes do
/// IsiVisitor. Desta forma, podemos extender o compilador muito facilmente,
/// sem ter código espaguete, e podemos atribuir ao parser *somente* a função
/// de criar a AST.
///
/// Esta trait contém uma função de visit para cada tipo de nó da AST.
///
/// Algumas funções já são fornecidas, especialmente nos nós que servem de
/// ramificação, como o Expression e o Statement.
pub trait IsiVisitor {
    type Ret;

    fn visit_int_literal(&mut self, lit: &IntLiteral) -> Self::Ret;

    fn visit_float_literal(&mut self, lit: &FloatLiteral) -> Self::Ret;

    fn visit_string_literal(&mut self, lit: &StringLiteral) -> Self::Ret;

    fn visit_ident(&mut self, id: &Ident) -> Self::Ret;

    fn visit_decl(&mut self, decl: &VarDecl) -> Self::Ret;

    fn visit_multi_decl(&mut self, decls: &MultiVarDecl) -> Self::Ret;

    fn visit_bin_expr(&mut self, bexpr: &BinExpr) -> Self::Ret;

    fn visit_expr(&mut self, expr: &Expr) -> Self::Ret {
        match expr {
            Expr::Ident(ident) => self.visit_ident(ident),
            Expr::ImmInt(imm) => self.visit_int_literal(imm),
            Expr::ImmFloat(imm) => self.visit_float_literal(imm),
            Expr::ImmString(imm) => self.visit_string_literal(imm),
            Expr::BinExpr(bexp) => self.visit_bin_expr(bexp),
            Expr::FnCall(call) => self.visit_fn_call(call),
            Expr::Negation(neg) => self.visit_negation(neg),
        }
    }

    fn visit_fn_call(&mut self, call: &FnCall) -> Self::Ret;

    fn visit_negation(&mut self, neg: &Negation) -> Self::Ret;

    fn visit_assignment(&mut self, assignment: &Assignment) -> Self::Ret;

    fn visit_conditional(&mut self, conditional: &Conditional) -> Self::Ret;

    fn visit_while_loop(&mut self, while_loop: &WhileLoop) -> Self::Ret;

    fn visit_do_while_loop(&mut self, do_while_loop: &DoWhileLoop) -> Self::Ret;

    fn visit_statement(&mut self, stmt: &Statement) -> Self::Ret {
        match stmt {
            Statement::Assignment(ass) => self.visit_assignment(ass),
            Statement::Decl(mdecl) => self.visit_multi_decl(mdecl),
            Statement::FnCall(call) => self.visit_fn_call(call),
            Statement::Conditional(cond) => self.visit_conditional(cond),
            Statement::WhileLoop(l) => self.visit_while_loop(l),
            Statement::DoWhileLoop(l) => self.visit_do_while_loop(l),
        }
    }

    fn visit_program(&mut self, program: &IsiProgram) -> Vec<Self::Ret> {
        let mut ret = vec![];

        for stmt in &program.statements {
            ret.push(self.visit_statement(stmt));
        }

        ret
    }
}
