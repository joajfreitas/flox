use lazy_static::lazy_static;
use regex::Regex;

use crate::chunk::{Chunk, OpCode, Value};
use crate::scanner::{Scanner, Token};

pub fn compile(source: &str, chunk: &mut Chunk) {
    let mut scanner = Scanner::new(source);

    parse(&mut scanner, chunk);
    chunk.write_opcode(OpCode::OpReturn, 1);
}

pub fn parse(scanner: &mut Scanner, chunk: &mut Chunk) {
    let token = scanner.peek().unwrap();

    match &token {
        Token::LeftParen => read_seq(scanner, chunk),
        Token::RightParen => {println!("unexpected ')'"); panic!()},
        Token::Atom(atom) => {
            dbg!(scanner.scan().unwrap());
            read_atom(&token, scanner, chunk);
        },
    }
}


fn unary(op: &str, scanner: &mut Scanner, chunk: &mut Chunk) {
    parse(scanner, chunk);
    match op {
        "not" => chunk.write_opcode(OpCode::OpNot, 1),
        _ => panic!(),
    }
}

fn binary(op: &str, scanner: &mut Scanner, chunk: &mut Chunk) {
    parse(scanner, chunk);
    parse(scanner, chunk);

    match op {
        "+" => chunk.write_opcode(OpCode::OpAdd, 1),
        "-" => chunk.write_opcode(OpCode::OpSubtract, 1),
        "*" => chunk.write_opcode(OpCode::OpMultiply, 1),
        "/" => chunk.write_opcode(OpCode::OpDivide, 1),
        "=" => chunk.write_opcode(OpCode::OpEq, 1),
        ">" => chunk.write_opcode(OpCode::OpBt, 1),
        ">=" => chunk.write_opcode(OpCode::OpBe, 1),
        "<" => chunk.write_opcode(OpCode::OpLt, 1),
        "<=" => chunk.write_opcode(OpCode::OpLe, 1),
        "and" => chunk.write_opcode(OpCode::OpAnd, 1),
        "nand" => chunk.write_opcode(OpCode::OpNand, 1),
        "or" => chunk.write_opcode(OpCode::OpOr, 1),
        "nor" => chunk.write_opcode(OpCode::OpNor, 1),
        "xor" => chunk.write_opcode(OpCode::OpXor, 1),
        "xnor" => chunk.write_opcode(OpCode::OpXnor, 1),
        _ => panic!(),
    };
}

pub fn read_seq(scanner: &mut Scanner, chunk: &mut Chunk) {
    let _ = scanner.scan();

    let op = scanner.peek().unwrap();
    println!("{:?}", op);
    match op {
        Token::Atom(ref atom) => read_atom(&op.clone(), scanner, chunk),
        Token::LeftParen => parse(scanner, chunk),
        _ => panic!(),
    };

    scanner.scan().unwrap();

}

fn read_atom(atom: &Token, scanner: &mut Scanner, chunk: &mut Chunk) {
    lazy_static! {
        static ref int_re: Regex = Regex::new(r"^-?[0-9]+$").unwrap();
        static ref str_re: Regex = Regex::new(r#""(?:\\.|[^\\"])*""#).unwrap();
    }

    let atom = match atom {
        Token::Atom(atom) => atom,
        _ => panic!(),
    };

    match dbg!(atom) as &str {
        "nil" => {
            chunk.write_opcode(OpCode::OpNil, 1);
            return;
        },
        "true" => {
            chunk.write_opcode(OpCode::OpTrue, 1);
            return;
        },
        "false" => {
            chunk.write_opcode(OpCode::OpFalse, 1);
            return;
        },
        "+" | "-" | "*" | "/" | "=" | "!=" | "<" | "<=" | ">" | ">=" | "and" | "nand" | "or" | "nor" | "xor" | "xnor"  => { 
            dbg!(scanner.scan().unwrap());
            binary(atom, scanner, chunk);
            return;
        },
        "not" => {
            scanner.scan().unwrap();
            unary(atom, scanner, chunk);
            return;
        },
        _ => {},
    }

    if int_re.is_match(&atom) {
        let i:i32 = atom.parse().unwrap();
        chunk.write_opcode(OpCode::OpConstant, 1);
        let constant = chunk.add_constant(Value::Number(i as f64));
        chunk.write_constant(constant as u8, 1);
        return;
    }

    panic!();
}

