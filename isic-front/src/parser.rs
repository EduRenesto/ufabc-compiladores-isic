use crate::ast;
use crate::span::Span;

peg::parser!{
    pub grammar isilang_parser() for str {
        rule spanned<T: std::fmt::Debug + PartialEq + Eq>(r: rule<T>) -> ast::Spanned<T>
            = start:position!() x:r() end:position!() {
                ast::Spanned {
                    span: Span { start, end },
                    node: x,
                }
            }

        pub rule num() -> ast::IntLiteral
            = t0:position!() n:$(['0'..='9']+) t1:position!() {
                ? {
                    let span = Span { start: t0, end: t1 };

                    n
                        .parse()
                        .map(|n| ast::IntLiteral(n, span))
                        .or(Err("u64"))
                }
            }

        pub rule numf() -> ast::FloatLiteral
            = t0:position!() n:$(['0'..='9']+ "," ['0'..='9']+) t1:position!() {
                ? {
                    let span = Span { start: t0, end: t1 };

                    n
                        .replace(",", ".")
                        .parse()
                        .map(|n| ast::FloatLiteral(n, span))
                        .or(Err("f32"))
                }
            }

        pub rule text() -> ast::StringLiteral
            = t0:position!() "\"" t:$(['a'..='z' | 'A'..='Z' | '0'..='9' | ' ']+) "\"" t1:position!() {
                let span = Span { start: t0, end: t1 };

                ast::StringLiteral(String::from(t), span)
            }

        pub rule ident() -> ast::Ident
            = t0:position!() id:$(['a'..='z' | 'A'..='Z']['a'..='z' | 'A'..='Z' | '0'..='9']*) t1:position!() {
                let span = Span { start: t0, end: t1 };

                ast::Ident::new(id, span)
            }

        // TODO(edu): varias variaveis num declare só
        pub rule decl() -> ast::VarDecl
            = t0:position!() "declare " vname:(ident()) (" "?) ":" (" "?) vtype:(ident()) "." t1:position!() {
                let span = Span { start: t0, end: t1 };
                ast::VarDecl::new(vname, vtype, span)
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
            lhs:(@) ws() "<" ws() rhs:@ { ast::Expr::BinExpr(ast::BinExpr(ast::BinaryOp::Lt, Box::new(lhs), Box::new(rhs))) }
            lhs:(@) ws() ">" ws() rhs:@ { ast::Expr::BinExpr(ast::BinExpr(ast::BinaryOp::Gt, Box::new(lhs), Box::new(rhs))) }
            lhs:(@) ws() "<=" ws() rhs:@ { ast::Expr::BinExpr(ast::BinExpr(ast::BinaryOp::Leq, Box::new(lhs), Box::new(rhs))) }
            lhs:(@) ws() ">=" ws() rhs:@ { ast::Expr::BinExpr(ast::BinExpr(ast::BinaryOp::Geq, Box::new(lhs), Box::new(rhs))) }
            lhs:(@) ws() "==" ws() rhs:@ { ast::Expr::BinExpr(ast::BinExpr(ast::BinaryOp::Eq, Box::new(lhs), Box::new(rhs))) }
            lhs:(@) ws() "!=" ws() rhs:@ { ast::Expr::BinExpr(ast::BinExpr(ast::BinaryOp::Neq, Box::new(lhs), Box::new(rhs))) }
            --
            f:numf() { ast::Expr::ImmFloat(f) }
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

        rule cond_taken_block() -> Vec<ast::Statement>
            = "entao" ws() "{" ws() stmts:(statement() ** ws()) ws() "}" {
                stmts
            }

        rule cond_not_taken_block() -> Vec<ast::Statement>
            = "senao" ws() "{" ws() stmts:(statement() ** ws()) ws() "}" {
                stmts
            }

        pub rule conditional() -> ast::Conditional
            = "se" ws() "(" ws() cond:expr() ws() ")" ws()
              taken:cond_taken_block() ws()
              not_taken:(cond_not_taken_block()?) {
                  let not_taken = match not_taken {
                      Some(stmts) => stmts,
                      None        => vec![],
                  };

                  ast::Conditional {
                      cond,
                      taken,
                      not_taken,
                  }
              }

        pub rule while_loop() -> ast::WhileLoop
            = "enquanto" ws() "(" ws() cond:expr() ")" ws() "{" ws() stmts:(statement() ** ws()) ws() "}" {
                ast::WhileLoop {
                    cond,
                    body: stmts,
                }
            }

        pub rule do_while_loop() -> ast::DoWhileLoop
            = "faca" ws() "{" ws() stmts:(statement() ** ws()) ws() "}" ws()
              "enquanto" ws() "(" ws() cond:expr() ws() ")." ws() {
                ast::DoWhileLoop {
                    cond,
                    body: stmts,
                }
            }

        pub rule statement() -> ast::Statement
            = d:decl()          { ast::Statement::Decl(d) }
            / fc:fncall()       { ast::Statement::FnCall(fc) }
            / a:assignment()    { ast::Statement::Assignment(a) }
            / c:conditional()   { ast::Statement::Conditional(c) }
            / l:while_loop()    { ast::Statement::WhileLoop(l) }
            / l:do_while_loop() { ast::Statement::DoWhileLoop(l) }

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
