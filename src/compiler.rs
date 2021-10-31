use lazy_static::lazy_static;
use regex::Regex;

use crate::chunk::{Chunk, OpCode, Value};
use crate::scanner::{Scanner, Token};

pub fn compile(source: &str, chunk: &mut Chunk) {
    let mut scanner = Scanner::new(source);

    parse(&mut scanner, chunk);
}

pub fn parse(scanner: &mut Scanner, chunk: &mut Chunk) {
    let token = scanner.peek().unwrap();

    match token {
        Token::LeftParen => read_seq(scanner, chunk),
        Token::RightParen => {println!("unexpected ')'"); panic!()},
        Token::Atom(atom) => println!("{:?}", atom),
    }
}

pub fn read_seq(scanner: &mut Scanner, chunk: &mut Chunk) {
    let _ = scanner.scan();

    let op = scanner.scan().unwrap();
    match op {
        Token::Atom("+") => chunk.write_opcode(OpCode::OpAdd, 1),
        Token::Atom("-") => chunk.write_opcode(OpCode::OpSubtract, 1),
        Token::Atom("*") => chunk.write_opcode(OpCode::OpMultiply, 1),
        Token::Atom("/") => chunk.write_opcode(OpCode::OpDivide, 1),
        _ => panic!(),
    };

    loop {
        let token = scanner.scan().unwrap();
        if token == Token::RightParen {
            break;
        }
        read_atom(token, chunk);
    }
    
}

fn read_atom(atom: Token, chunk: &mut Chunk) {
    lazy_static! {
        static ref int_re: Regex = Regex::new(r"^-?[0-9]+$").unwrap();
        static ref str_re: Regex = Regex::new(r#""(?:\\.|[^\\"])*""#).unwrap();
    }

    let atom = match atom {
        Token::Atom(atom) => atom,
        _ => panic!(),
    };

    if int_re.is_match(atom) {
        let i:i32 = atom.parse().unwrap();
        let constant = chunk.add_constant(Value::Value(i as f64));
        chunk.write_constant(constant as u8, 1);
        return;
    }

    panic!();
}

