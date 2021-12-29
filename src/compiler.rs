use lazy_static::lazy_static;
use regex::{Regex, Captures};
use rand::Rng;

use crate::chunk::{Chunk, OpCode, Value, Object, Closure};
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
        Token::Atom(_) => {
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
        "!=" => chunk.write_opcode(OpCode::OpNe, 1),
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
        Token::Atom(_) => read_atom(&op, scanner, chunk),
        Token::LeftParen => parse(scanner, chunk),
        _ => panic!(),
    };

    scanner.scan().unwrap();

}

fn unescape_str(s: &str) -> String {
    let re: Regex = Regex::new(r#"\\(.)"#).unwrap();
    re.replace_all(s, |caps: &Captures| {
        (if &caps[1] == "n" { "\n" } else { &caps[1] }).to_string()
    })
    .to_string()
}

fn read_shallow_list(scanner: &mut Scanner) -> Option<Vec<Token>> {
    assert!(scanner.scan().unwrap() == Token::LeftParen);
    let mut v: Vec<Token> = Vec::new();
    loop {
        let s = scanner.scan().unwrap();
        if s == Token::RightParen {
            break;
        }

        v.push(s);
    }

    Some(v)
}

fn parse_lambda(scanner: &mut Scanner) -> Option<Object> {
    assert!(scanner.scan().unwrap() == Token::Atom("lambda".to_string()));
    let args = dbg!(read_shallow_list(scanner).unwrap());

    let mut rng =  rand::thread_rng();
    let r: u32 = rng.gen();
    let name = format!("f{}", r);
    let mut closure = Closure {
        params: args.iter().map(|x| {x.atom()}).collect::<Vec<String>>(),
        chunk: Chunk::new(&name),
        name 
    };
    parse(scanner, &mut closure.chunk);
    Some(Object::Function(Box::new(closure)))
}

fn read_atom(atom: &Token, scanner: &mut Scanner, chunk: &mut Chunk) {
    lazy_static! {
        static ref INT_RE: Regex = Regex::new(r"^-?[0-9]+$").unwrap();
        static ref STR_RE: Regex = Regex::new(r#""(?:\\.|[^\\"])*""#).unwrap();
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
            chunk.write_opcode(OpCode::OpSetLocal, 0);
           
            let idx = chunk.add_constant(Value::Obj(Box::new(Object::Str(var_name))));
            chunk.write_constant(idx as u8, 1);
            return;
        },
        "if" => {
            scanner.scan().unwrap();
            parse(scanner, chunk);
            chunk.write_opcode(OpCode::OpJmpIfFalse, 1);
            chunk.write_constant(0, 1); //placeholder
            let branch_idx = chunk.get_current_index();
            parse(scanner,chunk);
            chunk.write_opcode(OpCode::OpJmp, 1);
            chunk.write_constant(0,1); //placeholder
            let jmp_idx = chunk.get_current_index();
            let false_idx = jmp_idx + 1;
            parse(scanner,chunk);
            let end_idx = chunk.get_current_index() + 1;

            chunk.rewrite_constant(branch_idx, false_idx as u8);
            chunk.rewrite_constant(jmp_idx, end_idx as u8);
            return;
        }
        "not" => {
            scanner.scan().unwrap();
            unary(atom, scanner, chunk);
            return;
        },
        "do" => {
            scanner.scan().unwrap();
            loop {
                if scanner.peek().unwrap() == Token::RightParen {
                    break
                }
                parse(scanner, chunk);
            }
            return;
        },
        "lambda" => {
            let lambda = dbg!(parse_lambda(scanner));
            panic!();
        }
        _ => {},
    }

    if INT_RE.is_match(atom) {
        let i:i32 = atom.parse().unwrap();
        chunk.write_opcode(OpCode::OpConstant, 1);
        let constant = chunk.add_constant(Value::Number(i as f64));
        chunk.write_constant(constant as u8, 1);
    }
    else if STR_RE.is_match(atom) {
        chunk.write_opcode(OpCode::OpConstant, 1);
        let s = Object::Str(unescape_str(&atom[1..atom.len() - 1]));
        let constant = chunk.add_constant(Value::Obj(Box::new(s)));
        chunk.write_constant(constant as u8, 1);
    }
    else {
        chunk.write_opcode(OpCode::OpGetLocal, 1);
        let idx = chunk.add_constant(Value::Obj(Box::new(Object::Str(atom.clone()))));
        chunk.write_constant(idx as u8, 1);
    }
}

