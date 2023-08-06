use crate::ast;

peg::parser!{
    pub grammar isilang_parser() for str {
        pub rule num() -> ast::IntLiteral
            = n:$(['0'..='9']+) {
                ? n.parse().map(|n| ast::IntLiteral(n)).or(Err("u64"))
            }

        pub rule ident() -> ast::Ident
            = id:$(['a'..='z' | 'A'..='Z']['a'..='z' | 'A'..='Z' | '0'..='9']*) {
                ast::Ident(String::from(id))
            }

        pub rule text() -> ast::StringLiteral
            = "\"" t:$(['a'..='z' | 'A'..='Z' | '0'..='9' | ' ']+) "\"" {
                ast::StringLiteral(String::from(t))
            }

        // TODO(edu): varias variaveis num declare só
        pub rule decl() -> ast::VarDecl
            = "declare " vname:(ident()) (" "?) ":" (" "?) vtype:(ident()) "." {
                ast::VarDecl::new(vname, vtype)
            }

        pub rule binop() -> ast::BinaryOp
            = "+"  { ast::BinaryOp::Add }
            / "-"  { ast::BinaryOp::Sub }
            / "*"  { ast::BinaryOp::Mul }
            / "/"  { ast::BinaryOp::Div }
            / ">=" { ast::BinaryOp::Geq }
            / "<=" { ast::BinaryOp::Leq }
            / ">"  { ast::BinaryOp::Gt }
            / "<"  { ast::BinaryOp::Lt }
            / "==" { ast::BinaryOp::Eq }
            / "!=" { ast::BinaryOp::Neq }

        pub rule expr() -> ast::Expr
            = fc:fncall() { ast::Expr::FnCall(fc) }
            / id:ident() { ast::Expr::Ident(id) }
            / n:num() { ast::Expr::ImmInt(n) }
            / t:text() { ast::Expr::ImmString(t) }
            // TODO: binary ops
            // / lhs:expr() ws() op:binop() ws() rhs:expr() {
            //     ast::Expr::BinExpr(
            //         op,
            //         Box::new(lhs),
            //         Box::new(rhs),
            //     )
            // }

        pub rule fncall() -> ast::FnCall
            = fname:ident() ws() "(" ws() args:(expr() ** (", " ws())) ws() ")." {
                ast::FnCall::new(fname, args)
            }

        pub rule assignment() -> ast::Assignment
            = id:ident() ws() ":=" ws() val:expr() ws() "." {
                ast::Assignment::new(id, val)
            }

        pub rule statement() -> ast::Statement
            = d:decl()       { ast::Statement::Decl(d) }
            / fc:fncall()    { ast::Statement::FnCall(fc) }
            / a:assignment() { ast::Statement::Assignment(a) }

        pub rule program() -> ast::IsiProgram
            = ws() "programa" ws() stmts:(statement() ** ws()) ws() "fimprog." ws() {
                ast::IsiProgram::new(stmts)
            };

        rule ws() = quiet!{ ([' ' | '\n' | '\t'])* }
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::ast;

    #[test]
    fn parse_num_ok() {
        let input = "42069";
        let ret = isilang_parser::num(&input);
        assert_eq!(ret, Ok(ast::IntLiteral(42069)));
    }

    #[test]
    fn parse_num_fail() {
        let input = "42069.0";
        let ret = isilang_parser::num(&input);
        assert!(ret.is_err());
    }

    #[test]
    fn parse_ident_ok() {
        let input = "foobar123";
        let ret = isilang_parser::ident(&input);
        assert_eq!(ret, Ok(ast::Ident("foobar123".to_string())));
    }

    #[test]
    fn parse_ident_fail() {
        let input = "123foobar";
        let ret = isilang_parser::ident(&input);
        assert!(ret.is_err());
    }

    #[test]
    fn parse_text_ok() {
        let input = r#""foo""#;
        let ret = isilang_parser::text(&input);
        assert_eq!(ret, Ok(ast::StringLiteral("foo".to_string())));
    }

    #[test]
    fn parse_text_fail() {
        let input = r#"foo""#;
        let ret = isilang_parser::text(&input);
        assert!(ret.is_err());
    }

    #[test]
    fn parse_decl_ok() {
        let input = "declare foo: int.";
        let ret = isilang_parser::decl(&input);

        let expected = ast::VarDecl::new(ast::Ident("foo".to_string()), ast::Ident("int".to_string()));

        assert_eq!(ret, Ok(expected));
    }

    #[test]
    fn parse_expr_ident_ok() {
        let input = "foo";
        let ret = isilang_parser::expr(&input);

        let expected = ast::Expr::Ident(ast::Ident("foo".to_string()));

        assert_eq!(ret, Ok(expected));
    }

    #[test]
    fn parse_expr_imm_int_ok() {
        let input = "123";
        let ret = isilang_parser::expr(&input);

        let expected = ast::Expr::ImmInt(ast::IntLiteral(123));

        assert_eq!(ret, Ok(expected));
    }

    #[test]
    fn parse_expr_str_int_ok() {
        let input = r#""foo""#;
        let ret = isilang_parser::expr(&input);

        let expected = ast::Expr::ImmString(ast::StringLiteral("foo".to_string()));

        assert_eq!(ret, Ok(expected));
    }

    #[test]
    fn parse_expr_fncall_ok() {
        let input = r#"foo(ident, 123, "text")."#;
        let ret = isilang_parser::expr(&input);

        let expected = ast::Expr::FnCall(ast::FnCall::new(
            ast::Ident("foo".to_string()),
            vec![
                ast::Expr::Ident(ast::Ident("ident".to_string())),
                ast::Expr::ImmInt(ast::IntLiteral(123)),
                ast::Expr::ImmString(ast::StringLiteral("text".to_string())),
            ],
        ));

        assert_eq!(ret, Ok(expected));
    }

    #[test]
    fn parse_fncall_ok() {
        let input = r#"foo(ident, 123, "text")."#;
        let ret = isilang_parser::fncall(&input);

        let expected = ast::FnCall::new(
            ast::Ident("foo".to_string()),
            vec![
                ast::Expr::Ident(ast::Ident("ident".to_string())),
                ast::Expr::ImmInt(ast::IntLiteral(123)),
                ast::Expr::ImmString(ast::StringLiteral("text".to_string())),
            ],
        );

        assert_eq!(ret, Ok(expected));
    }

    #[test]
    fn parse_assignment_ok() {
        let input = "foo := 123.";
        let ret = isilang_parser::assignment(&input);

        let expected = ast::Assignment::new(
            ast::Ident("foo".to_string()),
            ast::Expr::ImmInt(ast::IntLiteral(123)),
        );

        assert_eq!(ret, Ok(expected));
    }

    #[test]
    fn parse_program_ok() {
        let input = r"
            programa
                escreva(foo).

                declare foo: int.
                declare bar: string.

                foo := 123.
            fimprog.
        ";

        let ret = isilang_parser::program(&input);

        let expected = ast::IsiProgram::new(vec![
            ast::Statement::Decl(ast::VarDecl::new(ast::Ident("foo".to_string()), ast::Ident("int".to_string()))),
            ast::Statement::Decl(ast::VarDecl::new(ast::Ident("bar".to_string()), ast::Ident("string".to_string()))),
            ast::Statement::Assignment(ast::Assignment::new(ast::Ident("foo".to_string()), ast::Expr::ImmInt(ast::IntLiteral(123)))),
            ast::Statement::FnCall(ast::FnCall::new(ast::Ident("escreva".to_string()), vec![ast::Expr::Ident(ast::Ident("foo".to_string()))])),
        ]);

        assert_eq!(ret, Ok(expected));
    }
}
