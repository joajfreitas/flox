extern crate regex;
use lazy_static::lazy_static;
use regex::Regex;

use crate::scanner::{Scanner, Token, TokenType};

use super::ast::Ast;
use super::parser::L0Parser;

struct FloxL0Parser {
    ast: Ast,
}

impl FloxL0Parser {
    fn new() -> FloxL0Parser {
        FloxL0Parser { ast: Ast::Nil }
    }

    fn parse_ast(&mut self, scanner: &mut dyn Scanner<Item = Box<dyn Token>>) -> Ast {
        match scanner.token() {
            Some(token) => match token.token() {
                TokenType::LeftParen => self.read_seq(scanner, &TokenType::RightParen),
                TokenType::RightParen => panic!(),
                TokenType::Atom(_) => self.parse_atom(&*token),
            },
            None => Ast::nil(),
        }
    }

    fn read_seq(
        &mut self,
        scanner: &mut dyn Scanner<Item = Box<dyn Token>>,
        end: &TokenType,
    ) -> Ast {
        let mut seq: Vec<Ast> = Vec::new();
        loop {
            if scanner.peek().unwrap().token() == end {
                scanner.token();
                return Ast::list(seq);
            } else {
                seq.push(self.parse_ast(scanner));
            }
        }
    }

    fn parse_atom(&mut self, token: &dyn Token) -> Ast {
        lazy_static! {
            static ref INT_RE: Regex = Regex::new(r"^-?[0-9]+$").unwrap();
            static ref FLOAT_RE: Regex = Regex::new(r"^(-?)[0-9]+.[0-9]+$").unwrap();
            static ref STR_RE: Regex = Regex::new(r#""(?:\\.|[^\\"])*""#).unwrap();
        }

        let atom = token.token().get_atom();
        println!("{:?}", atom);

        if INT_RE.is_match(&atom) {
            Ast::int(atom.parse().expect("Failed to parse integer"))
        } else if FLOAT_RE.is_match(&atom) {
            Ast::double(atom.parse().expect("Failed to parse double"))
        } else if STR_RE.is_match(&atom) {
            Ast::Str(atom.clone())
        } else if atom == "nil" {
            Ast::Nil
        } else if atom == "true" {
            Ast::Bool(true)
        } else if atom == "false" {
            Ast::Bool(false)
        } else {
            Ast::Sym(atom.clone())
        }
    }
}

impl L0Parser for FloxL0Parser {
    fn parse(&mut self, scanner: &mut dyn Scanner<Item = Box<dyn Token>>) -> Vec<Ast> {
        let mut nodes: Vec<Ast> = Vec::new();

        if scanner.peek().is_none() {
            nodes.push(Ast::Nil);
            return nodes;
        }

        while let Some(node) = scanner.peek() {
            nodes.push(self.parse_ast(scanner));
        }

        return nodes;
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::scanner::FloxScanner;
    use rstest::*;

    #[fixture]
    fn parser() -> FloxL0Parser {
        FloxL0Parser::new()
    }

    #[fixture]
    fn scanner() -> FloxScanner {
        FloxScanner::new("")
    }

    #[test]
    fn test_init_ast() {
        assert_eq!(Ast::nil(), Ast::Nil);
        assert_eq!(Ast::bool(true), Ast::Bool(true));
        assert_eq!(Ast::int(1), Ast::Int(1));
        assert_eq!(Ast::double(1.0), Ast::Double(1.0));
        assert_eq!(Ast::list(vec![]), Ast::List(vec![]));
        assert_eq!(Ast::str("hi"), Ast::Str("hi".to_string()));
        assert_eq!(
            Ast::function("main", vec![]),
            Ast::Function("main".to_string(), vec![])
        );
    }

    #[test]
    fn test_list_recursion() {
        Ast::list(vec![Ast::int(1), Ast::int(2), Ast::int(3)]);
    }

    #[rstest]
    fn test_parse_empty_line(mut parser: FloxL0Parser, mut scanner: FloxScanner) {
        let ast = parser.parse(&mut scanner);
        assert_eq!(ast, vec![Ast::nil()])
    }

    #[rstest]
    fn test_parser_nil(mut parser: FloxL0Parser, mut scanner: FloxScanner) {
        scanner.feed("nil");
        let ast = parser.parse(&mut scanner);
        assert_eq!(ast, vec![Ast::Nil]);
    }

    #[rstest]
    fn test_parser_bool(mut parser: FloxL0Parser, mut scanner: FloxScanner) {
        scanner.feed("true");
        let ast = parser.parse(&mut scanner);
        assert_eq!(ast, vec![Ast::Bool(true)]);

        scanner.feed("false");
        let ast = parser.parse(&mut scanner);
        assert_eq!(ast, vec![Ast::Bool(false)]);
    }

    #[rstest]
    fn test_parser_int(mut parser: FloxL0Parser, mut scanner: FloxScanner) {
        scanner.feed("1");
        let ast = parser.parse(&mut scanner);
        assert_eq!(ast, vec![Ast::Int(1)]);
    }

    #[rstest]
    fn test_parser_double(mut parser: FloxL0Parser, mut scanner: FloxScanner) {
        scanner.feed("1.0");
        let ast = parser.parse(&mut scanner);
        assert_eq!(ast, vec![Ast::Double(1.0)]);
    }

    #[rstest]
    fn test_parser_sym(mut parser: FloxL0Parser, mut scanner: FloxScanner) {
        scanner.feed("\"hello world\"");
        let ast = parser.parse(&mut scanner);
        assert_eq!(ast, vec![Ast::Str("\"hello world\"".to_string())]);
    }

    #[rstest]
    fn test_parser_str(mut parser: FloxL0Parser, mut scanner: FloxScanner) {
        scanner.feed("\"hello world\"");
        let ast = parser.parse(&mut scanner);
        assert_eq!(ast, vec![Ast::Str("\"hello world\"".to_string())]);
    }

    #[rstest]
    fn test_parser_empty_list(mut parser: FloxL0Parser, mut scanner: FloxScanner) {
        let mut parser = FloxL0Parser::new();
        let mut scanner = FloxScanner::new("()");
        let ast = parser.parse(&mut scanner);
        assert_eq!(ast, vec![Ast::list(vec![])])
    }

    #[rstest]
    fn test_parser_list(mut parser: FloxL0Parser, mut scanner: FloxScanner) {
        scanner.feed("(+ 1 2)");
        let ast = parser.parse(&mut scanner);
        assert_eq!(
            ast,
            vec![Ast::list(vec![Ast::sym("+"), Ast::int(1), Ast::int(2)])]
        );
    }

    #[rstest]
    fn test_parser_multiple_list(mut parser: FloxL0Parser, mut scanner: FloxScanner) {
        scanner.feed("(+ 1 2)\n(* 2 2)");
        let ast = parser.parse(&mut scanner);
        assert_eq!(
            ast,
            vec![
                Ast::list(vec![Ast::sym("+"), Ast::int(1), Ast::int(2)]),
                Ast::list(vec![Ast::sym("*"), Ast::int(2), Ast::int(2)])
            ]
        );
    }
}
