use crate::ast;
use crate::span::Span;

peg::parser!{
    pub grammar isilang_parser() for str {
        rule spanned<T>(r: rule<T>) -> (T, Span)
            = start:position!() x:r() end:position!() {
                (x, Span { start, end })
            }

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

        //pub rule expr() -> ast::Expr
        //    = id:ident() { ast::Expr::Ident(id) }
        //    / n:num() { ast::Expr::ImmInt(n) }
        //    / t:text() { ast::Expr::ImmString(t) }

        //rule arith_operand() -> ast::ArithExpr
        //    = n:num() { ast::ArithExpr::ImmInt(n) }
        //    / id:ident() { ast::ArithExpr::Ident(id) }

        pub rule expr() -> ast::Expr = precedence!{
            lhs:(@) ws() "+" ws() rhs:@ { ast::Expr::BinExpr(ast::BinExpr(ast::BinaryOp::Add, Box::new(lhs), Box::new(rhs))) }
            lhs:(@) ws() "-" ws() rhs:@ { ast::Expr::BinExpr(ast::BinExpr(ast::BinaryOp::Sub, Box::new(lhs), Box::new(rhs))) }
            --
            lhs:(@) ws() "*" ws() rhs:@ { ast::Expr::BinExpr(ast::BinExpr(ast::BinaryOp::Mul, Box::new(lhs), Box::new(rhs))) }
            lhs:(@) ws() "/" ws() rhs:@ { ast::Expr::BinExpr(ast::BinExpr(ast::BinaryOp::Div, Box::new(lhs), Box::new(rhs))) }
            --
            n:num() { ast::Expr::ImmInt(n) }
            t:text() { ast::Expr::ImmString(t) }
            id:ident() { ast::Expr::Ident(id) }
            --
            "(" ws() e:expr() ws() ")" { e }
        }

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
    fn parse_expr_arith_ok() {
        let input = r#"foo * 1 + 3 / bar"#;
        let ret = isilang_parser::expr(&input);

        let expected = ast::Expr::BinExpr(
            ast::BinaryOp::Add,
            Box::new(ast::Expr::BinExpr(
                ast::BinaryOp::Mul,
                Box::new(ast::Expr::Ident(ast::Ident("foo".to_string()))),
                Box::new(ast::Expr::ImmInt(ast::IntLiteral(1))),
            )),
            Box::new(ast::Expr::BinExpr(
                ast::BinaryOp::Div,
                Box::new(ast::Expr::ImmInt(ast::IntLiteral(3))),
                Box::new(ast::Expr::Ident(ast::Ident("bar".to_string()))),
            )),
        );

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
            ast::Statement::FnCall(ast::FnCall::new(ast::Ident("escreva".to_string()), vec![ast::Expr::Ident(ast::Ident("foo".to_string()))])),
            ast::Statement::Decl(ast::VarDecl::new(ast::Ident("foo".to_string()), ast::Ident("int".to_string()))),
            ast::Statement::Decl(ast::VarDecl::new(ast::Ident("bar".to_string()), ast::Ident("string".to_string()))),
            ast::Statement::Assignment(ast::Assignment::new(ast::Ident("foo".to_string()), ast::Expr::ImmInt(ast::IntLiteral(123)))),
        ]);

        assert_eq!(ret, Ok(expected));
    }
}
