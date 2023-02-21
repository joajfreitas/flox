use crate::source_info::SourceInfo;
use crate::stage1::{S1, T1};
use serde::{Deserialize, Serialize};
use std::fmt;

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
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
                T2::List(ls) => format!("{}", S2s(ls.clone())),
                T2::Do(xs) => format!("({})", S2s(xs.clone())),
                T2::Lambda(args, body) =>
                    format!("( lambda {} {} )", S2s(args.clone()), dbg!(body)),
                T2::Defun(name, args, body) =>
                    format!("( defun {} {} {} )", name, S2s(args.clone()), dbg!(body)),
                T2::If(_, _, _) => "if".to_string(),
                T2::Set(_, _) => "set".to_string(),
            }
        )
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct S2 {
    t2: T2,
    source_info: SourceInfo,
}

impl PartialEq for S2 {
    fn eq(&self, other: &Self) -> bool {
        self.t2 == other.t2
    }
}

impl fmt::Display for S2 {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.t2)
    }
}

struct S2s(pub Vec<S2>);

impl fmt::Display for S2s {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "( {} )",
            self.0
                .iter()
                .map(|x| format!("{}", x))
                .intersperse(" ".to_string())
                .collect::<String>()
        )
    }
}

impl S2 {
    fn new(t2: &T2, source_info: &SourceInfo) -> S2 {
        S2 {
            t2: t2.clone(),
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
}

pub struct P2 {}

impl P2 {}

impl P2 {
    pub fn parse(input: &S1) -> Result<S2, String> {
        Ok(S2::new(
            &match &input.t1 {
                T1::Nil => T2::Nil,
                T1::Bool(b) => T2::Bool(*b),
                T1::Int(i) => T2::Int(*i),
                T1::Float(f) => T2::Float(*f),
                T1::Str(s) => T2::Str(s.to_string()),
                T1::Sym(s) => T2::Sym(s.to_string()),
                T1::List(_) => P2::parse_list(input)?.t2,
            },
            &input.source_info,
        ))
    }

    fn parse_list(input: &S1) -> Result<S2, String> {
        let input = input.get_list().ok_or("Cannot get list")?;

        Ok(match &input[0].get_sym().as_deref() {
            Some("do") => P2::parse_do(&input)?,
            Some("if") => P2::parse_if(&input)?,
            Some("lambda") => P2::parse_lambda(&input)?,
            Some("defun") => P2::parse_defun(&input)?,
            Some("set!") => P2::parse_set(&input)?,
            Some(&_) | None => S2::new(
                &T2::List(
                    input
                        .iter()
                        .map(|node| P2::parse(node).unwrap())
                        .collect::<Vec<S2>>(),
                ),
                &input[0].source_info,
            ),
        })
    }

    fn parse_do(input: &[S1]) -> Result<S2, String> {
        Ok(S2::new(
            &T2::Do(
                input[1..]
                    .iter()
                    .map(|x| P2::parse(x).unwrap())
                    .collect::<Vec<S2>>(),
            ),
            &input[0].source_info,
        ))
    }

    fn parse_if(input: &[S1]) -> Result<S2, String> {
        let stmt = &input[0];
        let pred = &input[1];
        let cond_true = &input[2];
        let cond_false = &input[3];
        Ok(S2::new(
            &T2::If(
                Box::new(P2::parse(pred)?),
                Box::new(P2::parse(cond_true)?),
                Box::new(P2::parse(cond_false)?),
            ),
            &stmt.source_info,
        ))
    }

    fn parse_lambda(input: &[S1]) -> Result<S2, String> {
        let lambda = &input[0];
        let args = &input[1];
        let body = &input[2];
        Ok(S2::new(
            &T2::Lambda(
                args.get_list()
                    .ok_or("Expected a list")?
                    .iter()
                    .map(|x| P2::parse(x).unwrap())
                    .collect::<Vec<S2>>(),
                Box::new(P2::parse(body)?),
            ),
            &lambda.source_info,
        ))
    }

    fn parse_defun(input: &[S1]) -> Result<S2, String> {
        let defun = &input[0];
        let name = &input[1];
        let args = &input[2];
        let body = &input[3];
        Ok(S2::new(
            &T2::Defun(
                Box::new(P2::parse(name)?),
                args.get_list()
                    .ok_or("Expected a list")?
                    .iter()
                    .map(|x| P2::parse(x).unwrap())
                    .collect::<Vec<S2>>(),
                Box::new(P2::parse(body)?),
            ),
            &defun.source_info,
        ))
    }
    fn parse_set(input: &[S1]) -> Result<S2, String> {
        let set = &input[0];
        let lvalue = &input[1];
        let rvalue = &input[2];

        Ok(S2::new(
            &T2::Set(Box::new(P2::parse(lvalue)?), Box::new(P2::parse(rvalue)?)),
            &set.source_info,
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
