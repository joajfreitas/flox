use crate::source_info::SourceInfo;
use crate::stage1::{S1, T1};
use std::fmt;

#[derive(Debug, Clone, PartialEq)]
enum T2 {
    Nil,
    Bool(bool),
    Int(i64),
    Float(f64),
    Str(String),
    Sym(String),
    List(Vec<S2>),
    Do(Vec<S2>),
    Lambda(Vec<S2>, Box<S2>),
    Defun(Box<S2>, Vec<S2>, Box<S2>),
    If(Box<S2>, Box<S2>, Box<S2>),
    Set(Box<S2>, Box<S2>),
}

impl fmt::Display for T2 {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{}",
            match self {
                T2::Nil => "nil".to_string(),
                T2::Bool(true) => "true".to_string(),
                T2::Bool(false) => "false".to_string(),
                T2::Int(i) => format!("{}", i),
                T2::Float(f) => format!("{}", f),
                T2::Str(s) => s.to_string(),
                T2::Sym(s) => s.to_string(),
                T2::List(ls) =>
                    "(".to_string()
                        + &itertools::intersperse(
                            ls.iter().map(|l| format!("{}", l)),
                            " ".to_string()
                        )
                        .collect::<String>()
                        + ")",
                T2::Do(_) => "do".to_string(),
                T2::Lambda(_, _) => "lambda".to_string(),
                T2::Defun(_, _, _) => "defun".to_string(),
                T2::If(_, _, _) => "if".to_string(),
                T2::Set(_, _) => "set".to_string(),
            }
        )
    }
}

#[derive(Debug, Clone)]
pub struct S2 {
    t1: T2,
    source_info: SourceInfo,
}

impl PartialEq for S2 {
    fn eq(&self, other: &Self) -> bool {
        self.t1 == other.t1
    }
}

impl fmt::Display for S2 {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.t1)
    }
}

impl S2 {
    fn new(t1: &T2, source_info: &SourceInfo) -> S2 {
        S2 {
            t1: t1.clone(),
            source_info: source_info.clone(),
        }
    }

    pub fn nil(source_info: &SourceInfo) -> S2 {
        S2::new(&T2::Nil, source_info)
    }

    pub fn bool(b: bool, source_info: &SourceInfo) -> S2 {
        S2::new(&T2::Bool(b), source_info)
    }

    pub fn int(i: i64, source_info: &SourceInfo) -> S2 {
        S2::new(&T2::Int(i), source_info)
    }

    pub fn float(f: f64, source_info: &SourceInfo) -> S2 {
        S2::new(&T2::Float(f), source_info)
    }

    pub fn str(s: &str, source_info: &SourceInfo) -> S2 {
        S2::new(&T2::Str(s.to_string()), source_info)
    }

    pub fn sym(s: &str, source_info: &SourceInfo) -> S2 {
        S2::new(&T2::Sym(s.to_string()), source_info)
    }

    pub fn list(l: Vec<S2>, source_info: &SourceInfo) -> S2 {
        S2::new(&T2::List(l), source_info)
    }

    pub fn lambda(args: Vec<S2>, source: S2, source_info: &SourceInfo) -> S2 {
        S2::new(&T2::Lambda(args, Box::new(source)), source_info)
    }

    fn get_sym(&self) -> String {
        match &self.t1 {
            T2::Sym(s) => s.to_string(),
            _ => panic!(),
        }
    }
}

pub struct P2 {}

impl P2 {}

impl P2 {
    pub fn parse(input: &S1) -> Result<S2, String> {
        Ok(S2::new(
            &match &input.ast_type {
                T1::Nil => T2::Nil,
                T1::Bool(b) => T2::Bool(*b),
                T1::Int(i) => T2::Int(*i),
                T1::Float(f) => T2::Float(*f),
                T1::Str(s) => T2::Str(s.to_string()),
                T1::Sym(s) => T2::Sym(s.to_string()),
                T1::List(_) => P2::parse_list(&input)?.t1,
            }
            .clone(),
            &input.source_info,
        ))
    }

    fn parse_list(input: &S1) -> Result<S2, String> {
        let list = input.get_list();

        let t1 = if !list[0].is_sym() {
            T2::List(
                input
                    .get_list()
                    .iter()
                    .map(|node| P2::parse(node).unwrap())
                    .collect::<Vec<S2>>(),
            )
        } else {
            match &list[0].get_sym() as &str {
                "do" => P2::parse_do(input)?.t1,
                "if" => P2::parse_if(input)?.t1,
                "lambda" => P2::parse_lambda(input)?.t1,
                "defun" => P2::parse_defun(input)?.t1,
                _ => panic!(),
            }
        };
        Ok(S2::new(&t1, &SourceInfo::default()))
    }

    fn parse_do(input: &S1) -> Result<S2, String> {
        Ok(S2::new(&T2::Do(vec![]), &SourceInfo::default()))
    }

    fn parse_if(input: &S1) -> Result<S2, String> {
        Ok(S2::new(
            &T2::If(
                Box::new(S2::int(1, &SourceInfo::default())),
                Box::new(S2::int(1, &SourceInfo::default())),
                Box::new(S2::int(1, &SourceInfo::default())),
            ),
            &SourceInfo::default(),
        ))
    }

    fn parse_lambda(input: &S1) -> Result<S2, String> {
        Ok(S2::new(
            &T2::Lambda(vec![], Box::new(S2::int(1, &SourceInfo::default()))),
            &SourceInfo::default(),
        ))
    }

    fn parse_defun(input: &S1) -> Result<S2, String> {
        Ok(S2::new(
            &T2::Defun(
                Box::new(S2::sym("f", &SourceInfo::default())),
                vec![],
                Box::new(S2::int(1, &SourceInfo::default())),
            ),
            &SourceInfo::default(),
        ))
    }
    fn parse_set(input: &S1) -> Result<S2, String> {
        Ok(S2::new(
            &T2::Set(
                Box::new(S2::sym("test", &SourceInfo::default())),
                Box::new(S2::int(1, &SourceInfo::default())),
            ),
            &SourceInfo::default(),
        ))
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::scanner::Scanner;
    use crate::stage1::P1;
    use rstest::rstest;

    #[test]
    fn test_create_s1_node() {
        assert_eq!(
            S2::int(32, &SourceInfo::default()),
            S2::int(32, &SourceInfo::default())
        );
    }

    #[test]
    fn test_parse_lambda() {
        assert_eq!(
            P2::parse(&S1::list(vec![S1::sym("lambda")])),
            Ok(S2::lambda(
                vec![],
                S2::int(1, &SourceInfo::default()),
                &SourceInfo::default()
            ))
        );
    }

    #[rstest]
    #[case("hello", Ok(S2::sym("hello", &SourceInfo::default())))]
    #[case("\"hello\"", Ok(S2::str("hello", &SourceInfo::default())))]
    #[case("1", Ok(S2::int(1, &SourceInfo::default())))]
    #[case("true", Ok(S2::bool(true, &SourceInfo::default())))]
    #[case("false", Ok(S2::bool(false, &SourceInfo::default())))]
    #[case("(1 2 3)", Ok(S2::list(vec![S2::int(1, &SourceInfo::default()), S2::int(2, &SourceInfo::default()), S2::int(3, &SourceInfo::default())], &SourceInfo::default())))]
    fn test_full_pipeline(#[case] input: &str, #[case] stage2: Result<S2, String>) {
        assert_eq!(
            P2::parse(&P1::parse(&mut Scanner::new(input)).unwrap()),
            stage2
        );
    }
}
