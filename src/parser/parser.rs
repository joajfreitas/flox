use crate::scanner::{Scanner, Token};

use super::ast::Ast;

pub trait L0Parser {
    fn parse(&mut self, scanner: &mut dyn Scanner<Item = Box<dyn Token>>) -> Vec<Ast>;
}

pub trait L1Parser {
    fn parse(&mut self, input: Vec<Ast>) -> Ast;
}

pub trait L2Parser {
    fn parse(&mut self, input: Ast) -> Ast;
}
