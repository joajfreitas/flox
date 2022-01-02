use lazy_static::lazy_static;
use regex::{Regex, Captures};
use rand::Rng;
use std::collections::HashMap;

use crate::chunk::{Chunk, OpCode, Value, Object, Closure};
use crate::scanner::{Scanner, Token};

struct Compiler {
    locals: Vec<String>,
}

impl Compiler {
    fn set_local(&mut self, name: String) -> usize {
        self.locals.push(name);
        self.locals.len() - 1

    }

    fn get_local(&mut self, name: String) -> Option<usize> {
        for (i, local) in self.locals.iter().enumerate().rev() {
            if local == &name {
                return Some(i);
            }
        }

        return None;
    }
}

pub fn compile(source: &str, chunk: &mut Chunk) -> Result<(), String> {
    let mut scanner = Scanner::new(source);
    let mut compiler = Compiler {
        locals: Vec::new(),
    };

    parse(&mut scanner, chunk, &mut compiler)?;
    chunk.write_opcode(OpCode::OpReturn, 1);

    Ok(())
}

fn parse(scanner: &mut Scanner, chunk: &mut Chunk, compiler: &mut Compiler) -> Result<(), String>{
    let token = match scanner.peek() {
        Some(x) => x,
        None => return Ok(()),
    };

    match &token {
        Token::LeftParen => read_seq(scanner, chunk, compiler)?,
        Token::RightParen => {return Err("unexpected ')'".to_string());},
        Token::Atom(_) => {
            scanner.scan().unwrap();
            read_atom(&token, scanner, chunk, compiler)?;
        },
    };

    Ok(())
}


fn unary(op: &str, scanner: &mut Scanner, chunk: &mut Chunk, compiler: &mut Compiler) -> Result<(), String>{
    parse(scanner, chunk, compiler)?;
    match op {
        "not" => chunk.write_opcode(OpCode::OpNot, 1),
        _ => {return Err(format!("Unexpected unary operation {}", op));},
    };

    Ok(())
}

fn binary(op: &str, scanner: &mut Scanner, chunk: &mut Chunk, compiler: &mut Compiler) -> Result<(), String>{
    parse(scanner, chunk, compiler)?;
    parse(scanner, chunk, compiler)?;

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
        _ => return Err(format!("Unexpected binary operation: {}", op)),
    };

    Ok(())
}

fn read_seq(scanner: &mut Scanner, chunk: &mut Chunk, compiler: &mut Compiler) -> Result<(), String>{
    let _ = scanner.scan();

    let op = scanner.peek().unwrap();
    match op {
        Token::Atom(_) => read_atom(&op, scanner, chunk, compiler)?,
        Token::LeftParen => parse(scanner, chunk, compiler)?,
        _ => {return Err(format!("unexpected token in sequence: {:?}", op));},
    };

    scanner.scan().unwrap();

    Ok(())

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

fn parse_lambda(scanner: &mut Scanner, compiler: &mut Compiler) -> Result<Object, String> {
    assert!(scanner.scan().unwrap() == Token::Atom("lambda".to_string()));
    let args = read_shallow_list(scanner).unwrap();

    let mut rng =  rand::thread_rng();
    let r: u32 = rng.gen();
    let name = format!("f{}", r);
    let mut closure = Closure {
        params: args.iter().map(|x| {x.atom()}).collect::<Vec<String>>(),
        chunk: Chunk::new(&name),
        name 
    };
    parse(scanner, &mut closure.chunk, compiler)?;
    Ok(Object::Function(Box::new(closure)))
}

fn read_atom(atom: &Token, scanner: &mut Scanner, chunk: &mut Chunk, compiler: &mut Compiler) -> Result<(), String>{
    lazy_static! {
        static ref INT_RE: Regex = Regex::new(r"^-?[0-9]+$").unwrap();
        static ref STR_RE: Regex = Regex::new(r#""(?:\\.|[^\\"])*""#).unwrap();
    }

    let atom = match atom {
        Token::Atom(atom) => atom,
        _ => {return Err(format!("Expected atom, got: {:?}", atom));},
    };

    match atom as &str {
        "nil" => {
            chunk.write_opcode(OpCode::OpNil, 1);
            return Ok(());
        },
        "true" => {
            chunk.write_opcode(OpCode::OpTrue, 1);
            return Ok(());
        },
        "false" => {
            chunk.write_opcode(OpCode::OpFalse, 1);
            return Ok(());
        },
        "+" | "-" | "*" | "/" | "=" | "!=" | "<" | "<=" | ">" | ">=" | "and" | "nand" | "or" | "nor" | "xor" | "xnor"  => { 
            scanner.scan().unwrap();
            binary(atom, scanner, chunk, compiler)?;
            return Ok(());
        },
        "set!" => {
            scanner.scan().unwrap(); //function name?
            let var_name = dbg!(scanner.scan().unwrap().atom()); //first arg
            parse(scanner, chunk, compiler)?;
            let idx = compiler.set_local(var_name.clone());
            chunk.write_opcode(OpCode::OpSetLocal, 0);
            chunk.write_constant(idx as u8, 0);
            return Ok(());
        },
        "if" => {
            scanner.scan().unwrap();
            parse(scanner, chunk, compiler)?;
            chunk.write_opcode(OpCode::OpJmpIfFalse, 1);
            chunk.write_constant(0, 1); //placeholder
            let branch_idx = chunk.get_current_index();
            parse(scanner,chunk, compiler)?;
            chunk.write_opcode(OpCode::OpJmp, 1);
            chunk.write_constant(0,1); //placeholder
            let jmp_idx = chunk.get_current_index();
            let false_idx = jmp_idx + 1;
            parse(scanner,chunk, compiler)?;
            let end_idx = chunk.get_current_index() + 1;

            chunk.rewrite_constant(branch_idx, false_idx as u8);
            chunk.rewrite_constant(jmp_idx, end_idx as u8);
            return Ok(());
        }
        "not" => {
            scanner.scan().unwrap();
            unary(atom, scanner, chunk, compiler)?;
            return Ok(());
        },
        "do" => {
            scanner.scan().unwrap();
            loop {
                if scanner.peek().unwrap() == Token::RightParen {
                    break
                }
                parse(scanner, chunk, compiler)?;
            }
            return Ok(());
        },
        "lambda" => {
            chunk.write_opcode(OpCode::OpConstant, 1);
            let lambda = dbg!(parse_lambda(scanner, compiler)?);
            let idx = chunk.add_constant(Value::Obj(Box::new(lambda)));
            chunk.write_constant(idx as u8, 1);
            return Ok(());
        },
        "apply" => {
            scanner.scan().unwrap();
            loop {
                if scanner.peek().unwrap() == Token::RightParen {
                    break
                }
                parse(scanner, chunk, compiler)?;
            }

            chunk.write_opcode(OpCode::OpCall, 1);
            return Ok(());
        },
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
        let idx = match compiler.get_local(atom.to_string()) {
            Some(idx) => idx,
            None => {
                return Err(format!("Symbol {} is not defined", atom));
            },
        };
        chunk.write_constant(idx as u8, 1);
    };

    Ok(())
}

