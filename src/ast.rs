use crate::scanner::{Scanner, Token};
use lazy_static::lazy_static;
use regex::Regex;

#[derive(Debug, PartialEq)]
pub enum Ast {
    Nil,
    Bool(bool),
    Int(i64),
    Float(f64),
    Str(String),
    Sym(String),
    List(Vec<Ast>),
}

pub struct Parser {
    scanner: Scanner,
}

impl Parser {
    fn new(scanner: &Scanner) -> Parser {
        Parser {
            scanner: scanner.clone(),
        }
    }

    fn parse(&mut self) -> Result<Ast, String> {
        let token = match self.scanner.peek() {
            Some(x) => x,
            None => return Ok(Ast::Nil),
        };

        match &token.0 {
            Token::LeftParen => self.parse_list(),
            Token::RightParen => Err("unexpected ')'".to_string()),
            Token::Atom(_) => self.parse_atom(),
        }
    }

    fn parse_list(&mut self) -> Result<Ast, String> {
        let _ = self.scanner.scan();

        let mut list: Vec<Ast> = Vec::new();

        while let Some(token) = self.scanner.peek() {
            let ast = match token.0 {
                Token::Atom(_) => self.parse_atom(),
                Token::LeftParen => self.parse(),
                Token::RightParen => {
                    self.scanner.scan();
                    break;
                }
            }?;
            list.push(ast);
            println!("{:?}", token);
        }

        Ok(Ast::List(list))
    }
    fn parse_atom(&mut self) -> Result<Ast, String> {
        lazy_static! {
            static ref INT_RE: Regex = Regex::new(r"^-?[0-9]+$").unwrap();
            static ref STR_RE: Regex = Regex::new(r#""(?:\\.|[^\\"])*""#).unwrap();
        }

        let atom = self.scanner.scan().unwrap();
        let atom = match &atom.0.atom()? as &str {
            "nil" => Ast::Nil,
            "true" => Ast::Bool(true),
            "false" => Ast::Bool(false),
            _ => {
                if INT_RE.is_match(&atom.0.atom()?) {
                    Ast::Int(atom.0.atom()?.parse().unwrap())
                } else {
                    Ast::Nil
                }
            }
        };

        Ok(atom)
    }
}

pub fn parse(source: &str) -> Result<Ast, String> {
    let scanner = Scanner::new(source);
    let mut parser = Parser::new(&scanner);
    parser.parse()
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::scanner::Scanner;

    #[test]
    fn test_create_ast_node() {
        assert_eq!(Ast::Int(32), Ast::Int(32));
    }

    #[test]
    fn test_parse() {
        assert_eq!(
            Parser::new(&mut Scanner::new("1")).parse().unwrap(),
            Ast::Int(1)
        )
    }
}
