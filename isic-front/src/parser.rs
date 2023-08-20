use crate::ast;
use crate::span::Span;

peg::parser! {
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

        rule decl() -> ast::VarDecl
            = t0:position!() vname:ident() ws() ":" ws() vtype:ident() t1:position!() {
                let span = Span { start: t0, end: t1 };

                ast::VarDecl::new(vname, vtype, span)
            }

        pub rule multidecl() -> ast::MultiVarDecl
            = "declare " ws() decls:(decl() ++ ("," ws())) ws() "." {
                ast::MultiVarDecl(decls)
            }

        pub rule negation() -> ast::Negation
            = t0:position!() "!" ws() t1:position!() e:expr() {
                let span = Span { start: t0, end: t1 };
                ast::Negation::new(Box::new(e), span)
            }

        pub rule expr() -> ast::Expr = precedence!{
            lhs:(@) ws() "&&" ws() rhs:@ { ast::Expr::BinExpr(ast::BinExpr(ast::BinaryOp::And, Box::new(lhs), Box::new(rhs))) }
            lhs:(@) ws() "||" ws() rhs:@ { ast::Expr::BinExpr(ast::BinExpr(ast::BinaryOp::Or, Box::new(lhs), Box::new(rhs))) }
            --
            lhs:(@) ws() "<" ws() rhs:@ { ast::Expr::BinExpr(ast::BinExpr(ast::BinaryOp::Lt, Box::new(lhs), Box::new(rhs))) }
            lhs:(@) ws() ">" ws() rhs:@ { ast::Expr::BinExpr(ast::BinExpr(ast::BinaryOp::Gt, Box::new(lhs), Box::new(rhs))) }
            lhs:(@) ws() "<=" ws() rhs:@ { ast::Expr::BinExpr(ast::BinExpr(ast::BinaryOp::Leq, Box::new(lhs), Box::new(rhs))) }
            lhs:(@) ws() ">=" ws() rhs:@ { ast::Expr::BinExpr(ast::BinExpr(ast::BinaryOp::Geq, Box::new(lhs), Box::new(rhs))) }
            lhs:(@) ws() "==" ws() rhs:@ { ast::Expr::BinExpr(ast::BinExpr(ast::BinaryOp::Eq, Box::new(lhs), Box::new(rhs))) }
            lhs:(@) ws() "!=" ws() rhs:@ { ast::Expr::BinExpr(ast::BinExpr(ast::BinaryOp::Neq, Box::new(lhs), Box::new(rhs))) }
            --
            lhs:(@) ws() "+" ws() rhs:@ { ast::Expr::BinExpr(ast::BinExpr(ast::BinaryOp::Add, Box::new(lhs), Box::new(rhs))) }
            lhs:(@) ws() "-" ws() rhs:@ { ast::Expr::BinExpr(ast::BinExpr(ast::BinaryOp::Sub, Box::new(lhs), Box::new(rhs))) }
            --
            lhs:(@) ws() "*" ws() rhs:@ { ast::Expr::BinExpr(ast::BinExpr(ast::BinaryOp::Mul, Box::new(lhs), Box::new(rhs))) }
            lhs:(@) ws() "/" ws() rhs:@ { ast::Expr::BinExpr(ast::BinExpr(ast::BinaryOp::Div, Box::new(lhs), Box::new(rhs))) }
            --
            lhs:(@) ws() "%" ws() rhs:@ { ast::Expr::BinExpr(ast::BinExpr(ast::BinaryOp::Mod, Box::new(lhs), Box::new(rhs))) }
            --
            f:numf() { ast::Expr::ImmFloat(f) }
            n:num() { ast::Expr::ImmInt(n) }
            t:text() { ast::Expr::ImmString(t) }
            id:ident() { ast::Expr::Ident(id) }
            --
            neg:negation() { ast::Expr::Negation(neg) }
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
            = d:multidecl()     { ast::Statement::Decl(d) }
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
