extern crate regex;
use regex::{Captures, Regex};

use crate::scanner::{FloxScanner, Scanner, Token, TokenType};

#[derive(Clone, Debug, PartialEq)]
pub enum Ast {
    Nil,
    Bool(bool),
    Int(i64),
    Double(f64),
    List(Vec<Ast>),
    Str(String),
    Sym(String),
    Function(String, Vec<String>),
}

impl Ast {
    pub fn nil() -> Ast {
        Ast::Nil
    }

    pub fn bool(b: bool) -> Ast {
        Ast::Bool(b)
    }

    pub fn int(i: i64) -> Ast {
        Ast::Int(i)
    }

    pub fn double(d: f64) -> Ast {
        Ast::Double(d)
    }

    pub fn list(l: Vec<Ast>) -> Ast {
        Ast::List(l)
    }

    pub fn str(s: &str) -> Ast {
        Ast::Str(s.to_string())
    }

    pub fn sym(s: &str) -> Ast {
        Ast::Sym(s.to_string())
    }

    pub fn function(source: &str, params: Vec<&str>) -> Ast {
        Ast::Function(
            source.to_string(),
            params.iter().map(|x| x.to_string()).collect(),
        )
    }
}

pub trait Parser {
    fn parse(&mut self, scanner: &mut dyn Scanner<Item = Box<dyn Token>>) -> Ast;
}

struct FloxParser {
    ast: Ast,
}

impl FloxParser {
    fn new() -> FloxParser {
        FloxParser { ast: Ast::Nil }
    }

    fn read_seq(
        &mut self,
        scanner: &mut dyn Scanner<Item = Box<dyn Token>>,
        end: &TokenType,
    ) -> Ast {
        let mut seq: Vec<Ast> = Vec::new();
        loop {
            if scanner.peek().unwrap().token() == end {
                return Ast::list(seq);
            } else {
                seq.push(self.parse(scanner));
            }
        }
    }

    fn parse_atom(&mut self, token: &dyn Token) -> Ast {
        let int_re: Regex = Regex::new(r"^-?[0-9]+$").unwrap();
        let str_re: Regex = Regex::new(r#""(?:\\.|[^\\"])*""#).unwrap();

        let atom = token.token().get_atom();
        if int_re.is_match(&atom) {
            Ast::int(atom.parse().expect("Failed to parse integer"))
        } else {
            Ast::Sym(atom.clone())
        }
    }
}

impl Parser for FloxParser {
    fn parse(&mut self, scanner: &mut Scanner<Item = Box<dyn Token>>) -> Ast {
        match scanner.token() {
            Some(token) => match token.token() {
                TokenType::LeftParen => self.read_seq(scanner, &TokenType::RightParen),
                TokenType::RightParen => Ast::nil(),
                TokenType::Atom(_) => self.parse_atom(&*token),
                _ => unimplemented!(),
            },
            None => Ast::nil(),
        }
    }
}

mod test {
    use super::*;

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

    #[test]
    fn test_parse_empty_line() {
        let mut parser = FloxParser::new();
        let mut scanner = FloxScanner::new("");
        let ast = parser.parse(&mut scanner);
        assert_eq!(ast, Ast::nil())
    }

    #[test]
    fn test_parser_empty_list() {
        let mut parser = FloxParser::new();
        let mut scanner = FloxScanner::new("()");
        let ast = parser.parse(&mut scanner);
        assert_eq!(ast, Ast::list(vec![]))
    }

    #[test]
    fn test_parser_sum() {
        let mut parser = FloxParser::new();
        let mut scanner = FloxScanner::new("(+ 1 2)");
        let ast = parser.parse(&mut scanner);
        assert_eq!(
            ast,
            Ast::list(vec![Ast::sym("+"), Ast::int(1), Ast::int(2)])
        );
    }
}
