use lazy_static::lazy_static;
use regex::{Regex, Captures};

use crate::chunk::{Chunk, OpCode, Value, Object};
use crate::scanner::{Scanner, Token};

pub fn compile(source: &str, chunk: &mut Chunk) {
    let mut scanner = Scanner::new(source);

    parse(&mut scanner, chunk);
    chunk.write_opcode(OpCode::OpReturn, 1);
}

pub fn parse(scanner: &mut Scanner, chunk: &mut Chunk) {
    let token = match scanner.peek() {
        Some(x) => x,
        None => return,
    };

    match &token {
        Token::LeftParen => read_seq(scanner, chunk),
        Token::RightParen => {println!("unexpected ')'"); panic!()},
        Token::Atom(atom) => {
            scanner.scan().unwrap();
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
    match op {
        Token::Atom(ref atom) => read_atom(&op.clone(), scanner, chunk),
        Token::LeftParen => parse(scanner, chunk),
        _ => panic!(),
    };

    scanner.scan().unwrap();

}

fn unescape_str(s: &str) -> String {
    let re: Regex = Regex::new(r#"\\(.)"#).unwrap();
    re.replace_all(&s, |caps: &Captures| {
        format!("{}", if &caps[1] == "n" { "\n" } else { &caps[1] })
    })
    .to_string()
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

    match atom as &str {
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
            scanner.scan().unwrap();
            binary(atom, scanner, chunk);
            return;
        },
        "set!" => {
            scanner.scan().unwrap(); //function name?
            let var_name = scanner.scan().unwrap().atom(); //first arg
            parse(scanner, chunk);
            chunk.write_opcode(OpCode::OpSetGlobal, 0);
           
            let idx = chunk.add_constant(Value::Obj(Box::new(Object::Str(var_name))));
            chunk.write_constant(idx as u8, 1);
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
    else if str_re.is_match(&atom) {
        chunk.write_opcode(OpCode::OpConstant, 1);
        let s = Object::Str(unescape_str(&atom[1..atom.len() - 1]));
        let constant = chunk.add_constant(Value::Obj(Box::new(s)));
        chunk.write_constant(constant as u8, 1);
        return;
    }
    else {
        chunk.write_opcode(OpCode::OpGetGlobal, 1);
        let idx = chunk.add_constant(Value::Obj(Box::new(Object::Str(atom.clone()))));
        chunk.write_constant(idx as u8, 1);
        return;
    }

    panic!();
}

