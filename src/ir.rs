use regex::Regex;
use lazy_static::lazy_static;

use crate::chunk::{Chunk, OpCode, Value};


pub struct IrScanner {
    tokens: Vec<String>,
    line: usize,
    pos: usize,
}

impl IrScanner {
    pub fn new(source: &str) -> IrScanner{
        IrScanner {
            tokens: tokenize(source),
            line: 0,
            pos: 0,
        }
    }

    pub fn scan(&mut self) -> Option<String> {
        let token = self.peek();
        self.pos = self.pos + 1;
        token
    }

    pub fn peek(&mut self) -> Option<String> {
        if self.pos >= self.tokens.len() {
            return None;
        }
        Some(self.tokens[self.pos].clone())
    }
}

fn _tokenize(source: &str) -> Vec<String> {
    lazy_static! {
        static ref RE:regex::Regex = Regex::new(r###"[\s,]*(~@|[\[\]{}()'`~^@]|"(?:\\.|[^\\"])*"?|;.*|[^\s\[\]{}('"`,;)]*)"###).unwrap();
    }

    let mut tokens: Vec<String> = Vec::new();
    for cap in RE.captures_iter(&source) {
        if cap[1].starts_with(";") || &cap[1] == "" {
            continue;
        }
        tokens.push(cap[1].to_string());
    }
    tokens
}


fn tokenize(source: &str) -> Vec<String> {
    let tokens = source.split("\n").map(|x| {
        let mut tokens = _tokenize(x);
        tokens.push("\n".to_string());
        tokens
    }).collect::<Vec<Vec<String>>>();

    let mut v: Vec<String> = Vec::new();

    for token in tokens {
        v.extend(token);
    }
    v
}

fn opcode_to_string(opcode: OpCode) -> &'static str {
    match opcode {
        OpCode::OpReturn => "RET",
        OpCode::OpConstant => "CONST" ,
        OpCode::OpConstantLong => "CONST_LONG",
        OpCode::OpSetGlobal => "SETGLOBAL",
        OpCode::OpGetGlobal => "GETGLOBAL",
        OpCode::OpAdd => "ADD",
        OpCode::OpSubtract => "SUB",
        OpCode::OpMultiply => "MUL",
        OpCode::OpDivide => "DIV",
        OpCode::OpNil => "NIL",
        OpCode::OpTrue => "TRUE",
        OpCode::OpFalse => "FALSE",
        OpCode::OpNot => "NOT",
        OpCode::OpEq => "EQ",
        OpCode::OpNe => "NE",
        OpCode::OpBt => "BT",
        OpCode::OpLt => "LT",
        OpCode::OpBe => "BE",
        OpCode::OpLe => "LE",
        OpCode::OpAnd => "AND",
        OpCode::OpNand => "NAND",
        OpCode::OpOr => "OR",
        OpCode::OpNor => "NOR",
        OpCode::OpXor => "XOR",
        OpCode::OpXnor => "XNOR",
        OpCode::OpJmpIfFalse => "JMPIF",
        OpCode::OpJmp => "JMP",
    }
}

fn string_to_opcode(s: &str) -> OpCode {
    match s {
         "RET"=> OpCode::OpReturn,
         "CONST" => OpCode::OpConstant,
         "CONST_LONG"=> OpCode::OpConstantLong,
         "SETGLOBAL"=> OpCode::OpSetGlobal,
         "GETGLOBAL"=> OpCode::OpGetGlobal,
         "ADD"=> OpCode::OpAdd,
         "SUB"=> OpCode::OpSubtract,
         "MUL"=> OpCode::OpMultiply,
         "DIV"=> OpCode::OpDivide,
         "NIL"=> OpCode::OpNil,
         "TRUE"=> OpCode::OpTrue,
         "FALSE"=> OpCode::OpFalse,
         "NOT"=> OpCode::OpNot,
         "EQ"=> OpCode::OpEq,
         "NE"=> OpCode::OpNe,
         "BT"=> OpCode::OpBt,
         "LT"=> OpCode::OpLt,
         "BE"=> OpCode::OpBe,
         "LE"=> OpCode::OpLe,
         "AND"=> OpCode::OpAnd,
         "NAND"=> OpCode::OpNand,
         "OR"=> OpCode::OpOr,
         "NOR"=> OpCode::OpNor,
         "XOR"=> OpCode::OpXor,
         "XNOR" => OpCode::OpXnor,
         "JMPIF" => OpCode::OpJmpIfFalse,
         "JMP" => OpCode::OpJmp,
         _ => panic!(),
    }

}

fn read_ir(content: &str) -> Option<Chunk> {
    let mut scanner = IrScanner::new(content);
    let mut chunk = Chunk::new("test");
   
    let mut state: u8 = 0;
    let mut line: u32 = 0;

    loop {
        let token = match scanner.scan() {
            Some(t) => t,
            None => break,
        };

        dbg!(state);
        if state == 0 {
            chunk.write_opcode(string_to_opcode(&token), line as usize);
            state = 1;
        } else if state == 1 {
            if token == "\n" {
                line+=1;
                state = 0;
            }
            else {
                chunk.write_constant(0, line as usize);
            }
        }
    }

    for element in chunk.get_code() {
        println!("{:?}", element);
    }
    println!("{}", chunk);
    return Some(chunk);
}

fn write_ir(chunk: &Chunk, filename: &str) -> Option<()>{
    None
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_tokenize() {
        assert_eq!(tokenize("RET 10 \"ola\""), vec!["RET", "10", "\"ola\"", "\n"]);
    }

    #[test]
    fn test_tokenize_with_newline() {
        assert_eq!(tokenize("RET 10 \"ola\"\nRET 20"), vec!["RET", "10", "\"ola\"", "\n", "RET", "20", "\n"]);
    }

    //#[test]
    //fn test_empty() {
    //    read_ir("CONST 2\nADD 0 0 \n RET");
    //}

}
