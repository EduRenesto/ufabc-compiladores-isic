use crate::ast;

peg::parser!{
    grammar isilang_parser() for str {
        pub rule num() -> ast::IntLiteral
            = n:$(['0'..='9']+) { ? n.parse().map(|n| ast::IntLiteral(n)).or(Err("u64")) }

        pub rule ident() -> ast::Ident
            = id:$(['a'..='z' | 'A'..='Z']['a'..='z' | 'A'..='Z' | '0'..='9']*) { ast::Ident(String::from(id)) }

        pub rule text() -> ast::StringLiteral
            = "\"" t:$(['a'..='z' | 'A'..='Z' | '0'..='9']+) "\"" {  ast::StringLiteral(String::from(t)) }
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
}
