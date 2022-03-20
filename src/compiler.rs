use lazy_static::lazy_static;
use rand::Rng;
use regex::{Captures, Regex};

use crate::chunk::closure::Closure;
use crate::chunk::object::Object;
use crate::chunk::value::Value;
use crate::chunk::{Chunk, OpCode};
use crate::scanner::{Scanner, Token};

#[derive(Clone)]
pub struct Compiler {
    context: String,
    locals: Vec<String>,
    up: Option<Box<Compiler>>,
    upvals: Vec<usize>,
}

impl Compiler {
    pub fn new(up: Option<Box<Compiler>>, context: &str) -> Compiler {
        Compiler {
            context: context.to_string(),
            locals: Vec::new(),
            up,
            upvals: Vec::new(),
        }
    }

    #[allow(dead_code)]
    fn resolve_upvalue(&mut self, name: &Token) -> Option<usize> {
        self.up.clone()?;
        let local = self.get_local(name.atom().unwrap());
        if !local.is_none() {
            return Some(self.add_upval(local.unwrap()));
        }

        None
    }

    #[allow(dead_code)]
    fn add_upval(&mut self, upvalue: usize) -> usize {
        self.upvals.push(upvalue);
        self.upvals.len()
    }

    fn set_local(&mut self, name: String) -> usize {
        self.locals.push(name);
        self.locals.len() - 1
    }

    fn get_local(&self, name: String) -> Option<usize> {
        //If the local has the same name has the context then we want to call the same function
        if self.context == name {
            return Some(0);
        }
        for (i, local) in self.locals.iter().enumerate().rev() {
            if local == &name {
                return Some(i);
            }
        }

        None
    }

    fn emit_nil(&self, chunk: &mut Chunk) -> Result<(), String> {
        chunk.write_opcode(OpCode::OpNil, 1);
        Ok(())
    }

    fn emit_true(&self, chunk: &mut Chunk) -> Result<(), String> {
        chunk.write_opcode(OpCode::OpTrue, 1);
        Ok(())
    }

    fn emit_false(&self, chunk: &mut Chunk) -> Result<(), String> {
        chunk.write_opcode(OpCode::OpFalse, 1);
        Ok(())
    }

    //fn emit_basic_algebra_operator(
    //    &mut self,
    //    chunk: &mut Chunk,
    //    atom: &str,
    //    scanner: &mut Scanner,
    //) -> Result<(), String> {
    //    scanner.scan().unwrap();
    //    binary(atom, scanner, chunk, self)?;
    //    Ok(())
    //}

    fn emit_binary_operation(
        &mut self,
        chunk: &mut Chunk,
        atom: &str,
        scanner: &mut Scanner,
    ) -> Result<(), String> {
        scanner.scan().unwrap();
        binary(atom, scanner, chunk, self)?;
        Ok(())
    }

    fn emit_set_local(&mut self, chunk: &mut Chunk, scanner: &mut Scanner) -> Result<(), String> {
        scanner.scan().unwrap(); //function name?
        let var_name = scanner.scan().unwrap().atom()?; //first arg
        parse(scanner, chunk, self)?;
        let idx = self.set_local(var_name);
        chunk.write_opcode(OpCode::OpSetLocal, 0);
        chunk.write_constant(idx as u8, 0);
        Ok(())
    }

    fn emit_if(&mut self, chunk: &mut Chunk, scanner: &mut Scanner) -> Result<(), String> {
        scanner.scan().unwrap();
        parse(scanner, chunk, self)?;
        chunk.write_opcode(OpCode::OpJmpIfFalse, 1);
        chunk.write_constant(0, 1); //placeholder
        let branch_idx = chunk.get_current_index()?;
        parse(scanner, chunk, self)?;
        chunk.write_opcode(OpCode::OpJmp, 1);
        chunk.write_constant(0, 1); //placeholder
        let jmp_idx = chunk.get_current_index()?;
        let false_idx = jmp_idx + 1;
        parse(scanner, chunk, self)?;
        let end_idx = chunk.get_current_index()? + 1;

        chunk.rewrite_constant(branch_idx, false_idx as u8);
        chunk.rewrite_constant(jmp_idx, end_idx as u8);
        Ok(())
    }

    fn emit_not(
        &mut self,
        chunk: &mut Chunk,
        atom: &str,
        scanner: &mut Scanner,
    ) -> Result<(), String> {
        scanner.scan().unwrap();
        unary(atom, scanner, chunk, self)?;
        Ok(())
    }

    fn emit_do(&mut self, chunk: &mut Chunk, scanner: &mut Scanner) -> Result<(), String> {
        scanner.scan().unwrap();
        loop {
            if scanner.peek().unwrap() == Token::RightParen {
                break;
            }
            parse(scanner, chunk, self)?;
        }
        Ok(())
    }

    fn emit_defun(&mut self, chunk: &mut Chunk, scanner: &mut Scanner) -> Result<(), String> {
        chunk.write_opcode(OpCode::OpConst, 1);
        let lambda = parse_defun(scanner, self)?;
        let idx = chunk.add_constant(Value::Obj(Box::new(lambda.clone())));
        chunk.write_constant(idx as u8, 1);
        let idx = self.set_local(dbg!(
            lambda.get_function().ok_or("Failed to find function")?.name
        ));
        chunk.write_opcode(OpCode::OpSetLocal, 0);
        chunk.write_constant(idx as u8, 0);

        Ok(())
    }

    fn emit_lambda(&mut self, chunk: &mut Chunk, scanner: &mut Scanner) -> Result<(), String> {
        chunk.write_opcode(OpCode::OpConst, 1);
        let lambda = parse_lambda(scanner, self)?;
        let idx = chunk.add_constant(Value::Obj(Box::new(lambda)));
        chunk.write_constant(idx as u8, 1);
        Ok(())
    }

    fn emit_integer(&self, chunk: &mut Chunk, atom: &str) -> Result<(), String> {
        let i: i32 = atom.parse().unwrap();
        chunk.write_opcode(OpCode::OpConst, 1);
        let constant = chunk.add_constant(Value::Number(i as f64));
        chunk.write_constant(constant as u8, 1);
        Ok(())
    }
    fn emit_string(&self, chunk: &mut Chunk, atom: &str) -> Result<(), String> {
        chunk.write_opcode(OpCode::OpConst, 1);
        let s = Object::Str(unescape_str(&atom[1..atom.len() - 1]));
        let constant = chunk.add_constant(Value::Obj(Box::new(s)));
        chunk.write_constant(constant as u8, 1);
        Ok(())
    }

    fn emit_get_local(&self, chunk: &mut Chunk, atom: &str) -> Result<(), String> {
        chunk.write_opcode(OpCode::OpGetLocal, 1);
        let idx = match self.get_local(atom.to_string()) {
            Some(idx) => idx,
            None => {
                return Err(format!("Symbol {} is not defined", atom));
            }
        };
        chunk.write_constant(idx as u8, 1);
        Ok(())
    }

    fn emit_function_call(
        &mut self,
        chunk: &mut Chunk,
        atom: &str,
        scanner: &mut Scanner,
    ) -> Result<(), String> {
        scanner.scan().unwrap();
        chunk.write_opcode(OpCode::OpGetLocal, 1);
        let idx = match self.get_local(atom.to_string()) {
            Some(idx) => idx,
            None => {
                return Err(format!("Symbol {} is not defined", atom));
            }
        };
        chunk.write_constant(idx as u8, 1);
        loop {
            if scanner.peek().unwrap() == Token::RightParen {
                break;
            }
            parse(scanner, chunk, self)?;
        }

        chunk.write_opcode(OpCode::OpCall, 1);
        Ok(())
    }
}

pub fn compile(source: &str, chunk: &mut Chunk, compiler: &mut Compiler) -> Result<(), String> {
    let mut scanner = Scanner::new(source);
    parse(&mut scanner, chunk, compiler)?;
    chunk.write_opcode(OpCode::OpRet, 1);

    Ok(())
}

fn parse(scanner: &mut Scanner, chunk: &mut Chunk, compiler: &mut Compiler) -> Result<(), String> {
    let token = match scanner.peek() {
        Some(x) => x,
        None => return Ok(()),
    };

    match &token {
        Token::LeftParen => read_seq(scanner, chunk, compiler)?,
        Token::RightParen => {
            return Err("unexpected ')'".to_string());
        }
        Token::Atom(_) => {
            scanner.scan().unwrap();
            read_atom(&token, scanner, chunk, compiler)?;
        }
    };

    Ok(())
}

fn unary(
    op: &str,
    scanner: &mut Scanner,
    chunk: &mut Chunk,
    compiler: &mut Compiler,
) -> Result<(), String> {
    parse(scanner, chunk, compiler)?;
    match op {
        "not" => chunk.write_opcode(OpCode::OpNot, 1),
        _ => {
            return Err(format!("Unexpected unary operation {}", op));
        }
    };

    Ok(())
}

fn binary(
    op: &str,
    scanner: &mut Scanner,
    chunk: &mut Chunk,
    compiler: &mut Compiler,
) -> Result<(), String> {
    parse(scanner, chunk, compiler)?;
    parse(scanner, chunk, compiler)?;

    match op {
        "+" => chunk.write_opcode(OpCode::OpAdd, 1),
        "-" => chunk.write_opcode(OpCode::OpSub, 1),
        "*" => chunk.write_opcode(OpCode::OpMul, 1),
        "/" => chunk.write_opcode(OpCode::OpDiv, 1),
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

fn read_seq(
    scanner: &mut Scanner,
    chunk: &mut Chunk,
    compiler: &mut Compiler,
) -> Result<(), String> {
    let _ = scanner.scan();

    let op = scanner.peek().unwrap();
    match op {
        Token::Atom(_) => read_atom(&op, scanner, chunk, compiler)?,
        Token::LeftParen => parse(scanner, chunk, compiler)?,
        _ => {
            return Err(format!("unexpected token in sequence: {:?}", op));
        }
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
    let mut rng = rand::thread_rng();
    let r: u32 = rng.gen();
    let name = format!("f{}", r);
    let mut closure = Closure {
        params: args
            .iter()
            .map(|x| x.atom().unwrap())
            .collect::<Vec<String>>(),
        chunk: Chunk::new(&name),
        name,
    };

    let mut compiler = Compiler::new(Some(Box::new((*compiler).clone())), "lambda");

    for arg in args {
        compiler.set_local(arg.atom()?);
    }
    parse(scanner, &mut closure.chunk, &mut compiler)?;
    closure.chunk.write_opcode(OpCode::OpRet, 1);
    Ok(Object::Function(Box::new(closure)))
}

fn parse_defun(scanner: &mut Scanner, compiler: &mut Compiler) -> Result<Object, String> {
    assert!(scanner.scan().unwrap() == Token::Atom("defun".to_string()));
    let name = scanner.scan().unwrap().atom().unwrap();
    let args = read_shallow_list(scanner).unwrap();
    let mut closure = Closure {
        params: args
            .iter()
            .map(|x| x.atom().unwrap())
            .collect::<Vec<String>>(),
        chunk: Chunk::new(&name),
        name: name.clone(),
    };

    let mut compiler = Compiler::new(Some(Box::new((*compiler).clone())), &name);

    for arg in args {
        compiler.set_local(arg.atom()?);
    }
    parse(scanner, &mut closure.chunk, &mut compiler)?;
    closure.chunk.write_opcode(OpCode::OpRet, 1);
    println!("{}", closure.chunk);
    Ok(Object::Function(Box::new(closure)))
}

fn read_atom(
    atom: &Token,
    scanner: &mut Scanner,
    chunk: &mut Chunk,
    compiler: &mut Compiler,
) -> Result<(), String> {
    lazy_static! {
        static ref INT_RE: Regex = Regex::new(r"^-?[0-9]+$").unwrap();
        static ref STR_RE: Regex = Regex::new(r#""(?:\\.|[^\\"])*""#).unwrap();
    }

    let atom: &str = &atom.atom()?;

    match atom as &str {
        "nil" => compiler.emit_nil(chunk),
        "true" => compiler.emit_true(chunk),
        "false" => compiler.emit_false(chunk),
        "+" | "-" | "*" | "/" | "=" | "!=" | "<" | "<=" | ">" | ">=" | "and" | "nand" | "or"
        | "nor" | "xor" | "xnor" => compiler.emit_binary_operation(chunk, atom, scanner),
        "set!" => compiler.emit_set_local(chunk, scanner),
        "if" => compiler.emit_if(chunk, scanner),
        "not" => compiler.emit_not(chunk, atom, scanner),
        "do" => compiler.emit_do(chunk, scanner),
        "lambda" => compiler.emit_lambda(chunk, scanner),
        "defun" => compiler.emit_defun(chunk, scanner),
        _ => {
            if INT_RE.is_match(atom) {
                compiler.emit_integer(chunk, atom)
            } else if STR_RE.is_match(atom) {
                compiler.emit_string(chunk, atom)
            } else if scanner.previous() != Some(Token::LeftParen) {
                compiler.emit_get_local(chunk, atom)
            } else {
                compiler.emit_function_call(chunk, atom, scanner)
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use rstest::*;

    use crate::chunk::Element;
    use crate::{constant, op};

    #[fixture]
    fn compiler() -> Compiler {
        Compiler::new(None, "main")
    }

    #[fixture]
    fn chunk() -> Chunk {
        Chunk::new("main")
    }

    #[rstest]
    fn test_locals(mut compiler: Compiler) {
        compiler.set_local(String::from("x"));
        compiler.set_local(String::from("y"));
        assert_eq!(compiler.get_local(String::from("x")), Some(0));
        assert_eq!(compiler.get_local(String::from("y")), Some(1));
    }

    #[rstest]
    fn test_local_same_name_as_module(compiler: Compiler) {
        assert_eq!(Some(0), compiler.get_local(String::from("main")));
    }

    #[rstest]
    fn test_emit_nil(compiler: Compiler, mut chunk: Chunk) {
        compiler.emit_nil(&mut chunk).unwrap();
        assert_eq!(chunk.get_code(), vec![op!(OpCode::OpNil)]);
    }

    #[rstest]
    fn test_emit_true(compiler: Compiler, mut chunk: Chunk) {
        compiler.emit_true(&mut chunk).unwrap();
        assert_eq!(chunk.get_code(), vec![op!(OpCode::OpTrue)]);
    }

    #[rstest]
    fn test_emit_false(compiler: Compiler, mut chunk: Chunk) {
        compiler.emit_false(&mut chunk).unwrap();
        assert_eq!(chunk.get_code(), vec![op!(OpCode::OpFalse)]);
    }

    #[rstest]
    fn test_compile(mut compiler: Compiler, mut chunk: Chunk) {
        compile("1", &mut chunk, &mut compiler);
        assert_eq!(
            chunk.get_code(),
            vec![op!(OpCode::OpConst), constant!(0), op!(OpCode::OpRet)]
        );
    }
}
