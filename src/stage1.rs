//! Simple ast representation.
//!
//! Can represent integers, floats, strings, symbols and lists.

use crate::scanner::{Scanner, Token};
use crate::source_info::SourceInfo;
use itertools;
use lazy_static::lazy_static;
use regex::Regex;
use serde::{Deserialize, Serialize};
use std::fmt;

#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
pub enum T1 {
    Nil,
    Bool(bool),
    Int(i64),
    Float(f64),
    Str(String),
    Sym(String),
    List(Vec<S1>),
}

impl fmt::Display for T1 {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{}",
            match self {
                T1::Nil => "nil".to_string(),
                T1::Bool(true) => "true".to_string(),
                T1::Bool(false) => "false".to_string(),
                T1::Int(i) => format!("{}", i),
                T1::Float(f) => format!("{}", f),
                T1::Str(s) => s.to_string(),
                T1::Sym(s) => s.to_string(),
                T1::List(ls) =>
                    "(".to_string()
                        + &itertools::intersperse(
                            ls.iter().map(|l| format!("{}", l)),
                            " ".to_string()
                        )
                        .collect::<String>()
                        + ")",
            }
        )
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct S1 {
    pub t1: T1,
    pub source_info: SourceInfo,
}

impl PartialEq for S1 {
    fn eq(&self, other: &Self) -> bool {
        self.t1 == other.t1
    }
}

impl fmt::Display for S1 {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.t1)
    }
}

impl S1 {
    fn new(t1: T1) -> S1 {
        S1 {
            t1,
            source_info: SourceInfo::default(),
        }
    }

    pub fn nil() -> S1 {
        S1::new(T1::Nil)
    }

    pub fn boolean(b: bool) -> S1 {
        S1::new(T1::Bool(b))
    }

    pub fn int(i: i64) -> S1 {
        S1::new(T1::Int(i))
    }

    pub fn list(l: Vec<S1>) -> S1 {
        S1::new(T1::List(l))
    }

    pub fn sym(symbol: &str) -> S1 {
        S1::new(T1::Sym(symbol.to_string()))
    }

    pub fn str(string: &str) -> S1 {
        S1::new(T1::Str(string.to_string()))
    }

    pub fn get_sym(&self) -> Option<String> {
        match &self.t1 {
            T1::Sym(s) => Some(s.to_string()),
            _ => None,
        }
    }

    pub fn get_list(&self) -> Option<Vec<S1>> {
        match &self.t1 {
            T1::List(l) => Some(l.clone()),
            _ => None,
        }
    }

    pub fn is_sym(&self) -> bool {
        matches!(self.t1, T1::Sym(_))
    }

    pub fn is_list(&self) -> bool {
        matches!(self.t1, T1::List(_))
    }
}

impl S1 {}

pub struct P1 {}

impl P1 {
    pub fn parse(scanner: &mut Scanner) -> Result<S1, String> {
        let token = match scanner.peek() {
            Some(x) => x,
            None => return Ok(S1::nil()),
        };

        match &token.0 {
            Token::LeftParen => P1::parse_list(scanner),
            Token::RightParen => Err("unexpected ')'".to_string()),
            Token::Atom(_) => P1::parse_atom(scanner),
        }
    }

    fn parse_list(scanner: &mut Scanner) -> Result<S1, String> {
        let _ = scanner.scan();

        let mut list: Vec<S1> = Vec::new();

        while let Some(token) = scanner.peek() {
            let ast = match token.0 {
                Token::Atom(_) => P1::parse_atom(scanner),
                Token::LeftParen => P1::parse(scanner),
                Token::RightParen => {
                    scanner.scan();
                    break;
                }
            }?;
            list.push(ast);
        }

        Ok(S1::list(list))
    }
    fn parse_atom(scanner: &mut Scanner) -> Result<S1, String> {
        lazy_static! {
            static ref INT_RE: Regex = Regex::new(r"^-?[0-9]+$").unwrap();
            static ref STR_RE: Regex = Regex::new(r#""((\\.|[^\\"])*)""#).unwrap();
        }

        let atom: &str = &scanner
            .scan()
            .ok_or("Unexpected end of line".to_string())?
            .0
            .atom()?;

        let atom = match atom {
            "nil" => S1::nil(),
            "true" => S1::boolean(true),
            "false" => S1::boolean(false),
            _ => {
                if INT_RE.is_match(atom) {
                    S1::int(atom.parse().unwrap())
                } else if STR_RE.is_match(atom) {
                    S1::str(STR_RE.captures(atom).unwrap().get(1).unwrap().as_str())
                } else {
                    S1::sym(atom)
                }
            }
        };

        Ok(atom)
    }
}

pub fn parse(source: &str) -> Result<S1, String> {
    P1::parse(&mut Scanner::new(source))
}

#[cfg(test)]
mod test {
    use super::*;
    use rstest::rstest;

    #[test]
    fn test_create_ast_node() {
        assert_eq!(S1::int(32), S1::int(32));
    }

    #[test]
    fn test_parse() {
        assert_eq!(parse("1").unwrap(), S1::int(1))
    }

    #[test]
    fn test_nil() {
        assert_eq!(parse("nil"), Ok(S1::nil()))
    }

    #[test]
    fn test_true() {
        assert_eq!(parse("true"), Ok(S1::boolean(true)))
    }

    #[test]
    fn test_false() {
        assert_eq!(parse("false"), Ok(S1::boolean(false)))
    }

    #[test]
    fn test_function_call() {
        assert_eq!(
            parse("(+ 1 2)"),
            Ok(S1::list(vec![S1::sym("+"), S1::int(1), S1::int(2)]))
        )
    }

    #[rstest]
    #[case("\"hello\"", "hello")]
    //#[case(r#""\"hello\"""#, "\"hello\"")]
    fn test_str(#[case] string1: &str, #[case] string2: &str) {
        assert_eq!(parse(string1), Ok(S1::str(string2)))
    }
}
