use lazy_static::lazy_static;
use rand::Rng;
use regex::{Captures, Regex};
use std::fmt;

use crate::chunk::object::{Function, Object};
use crate::chunk::value::Value;
use crate::chunk::{Chunk, OpCode};
use crate::scanner::{Scanner, Token};

#[derive(Clone)]
pub struct UpValue {
    is_local: bool,
    pub index: usize,
}

impl UpValue {
    pub fn new(is_local: bool, index: usize) -> UpValue {
        UpValue { is_local, index }
    }
}

impl fmt::Debug for UpValue {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "(upvalue {})", self.index)
    }
}

#[derive(Clone)]
pub enum Ctx {
    TopLevel,
    FunctionScope(String),
}

#[derive(Clone)]
pub struct Compiler {
    context: Ctx,
    locals: Vec<String>,
    up: Option<Box<Compiler>>,
    upvals: Vec<UpValue>,
}

impl Compiler {
    pub fn new(up: Option<Box<Compiler>>, context: Ctx) -> Compiler {
        Compiler {
            context,
            locals: Vec::new(),
            up,
            upvals: Vec::new(),
        }
    }

    fn set_local(&mut self, name: String) -> usize {
        self.locals.push(name);
        self.locals.len() - 1
    }

    fn get_local(&self, name: &str) -> Option<usize> {
        for (i, local) in self.locals.iter().enumerate().rev() {
            if local == name {
                return Some(i);
            }
        }

        None
    }

    fn get_upvalue(&self, name: &str) -> Option<usize> {
        let up = self.up.as_ref()?;

        if let Some(id) = dbg!(up.get_local(name)) {
            return Some(id);
        }

        up.get_upvalue(name)
    }

    fn add_upvalue(&mut self, id: usize, is_local: bool) -> Option<usize> {
        for upval in self.upvals.iter() {
            if upval.index == id && upval.is_local == is_local {
                return Some(upval.index);
            }
        }
        self.upvals.push(UpValue::new(is_local, id));
        Some(self.upvals.len())
    }

    fn emit_nil(&self, chunk: &mut Chunk, line: usize) -> Result<(), String> {
        chunk.write_opcode(OpCode::OpNil, line);
        Ok(())
    }

    fn emit_true(&self, chunk: &mut Chunk, line: usize) -> Result<(), String> {
        chunk.write_opcode(OpCode::OpTrue, line);
        Ok(())
    }

    fn emit_false(&self, chunk: &mut Chunk, line: usize) -> Result<(), String> {
        chunk.write_opcode(OpCode::OpFalse, line);
        Ok(())
    }

    fn emit_print(
        &mut self,
        chunk: &mut Chunk,
        atom: (Token, usize),
        scanner: &mut Scanner,
    ) -> Result<(), String> {
        scanner.scan().unwrap();
        parse(scanner, chunk, self)?;
        chunk.write_opcode(OpCode::OpPrint, atom.1);
        Ok(())
    }

    fn emit_binary_operation(
        &mut self,
        chunk: &mut Chunk,
        atom: (Token, usize),
        scanner: &mut Scanner,
    ) -> Result<(), String> {
        binary(atom, scanner, chunk, self)?;
        Ok(())
    }

    fn emit_set_local(&mut self, chunk: &mut Chunk, name: &(Token, usize)) -> Result<(), String> {
        //scanner.scan().unwrap(); //function name?
        //let var_name = scanner.scan().unwrap().atom()?; //first arg
        //parse(scanner, chunk, self)?;
        let idx = self.set_local(name.0.atom()?);
        chunk.write_opcode(OpCode::OpSetLocal, name.1);
        chunk.write_constant(idx as u8, name.1);
        Ok(())
    }

    fn emit_set_upvalue(&mut self, chunk: &mut Chunk, name: &str) -> Result<(), String> {
        //scanner.scan().unwrap(); //function name?
        //let var_name = scanner.scan().unwrap().atom()?; //first arg
        //parse(scanner, chunk, self)?;
        if let Some(id) = self.get_upvalue(name) {
            chunk.write_opcode(OpCode::OpSetUpvalue, 0);
            chunk.write_constant(id as u8, 0);
            return Ok(());
        }

        Err(String::from("Failed to set upvalue"))
    }

    fn emit_if(&mut self, chunk: &mut Chunk, scanner: &mut Scanner) -> Result<(), String> {
        let (_, line) = scanner.scan().unwrap();
        parse(scanner, chunk, self)?;
        chunk.write_opcode(OpCode::OpJmpIfFalse, line);
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
        atom: (Token, usize),
        scanner: &mut Scanner,
    ) -> Result<(), String> {
        scanner.scan().unwrap();
        unary(atom, scanner, chunk, self)?;
        Ok(())
    }

    fn emit_do(&mut self, chunk: &mut Chunk, scanner: &mut Scanner) -> Result<(), String> {
        scanner.scan().unwrap();
        loop {
            if scanner.peek().unwrap().0 == Token::RightParen {
                break;
            }
            parse(scanner, chunk, self)?;
        }
        Ok(())
    }

    //fn emit_defun(&mut self, chunk: &mut Chunk, scanner: &mut Scanner) -> Result<(), String> {
    //    chunk.write_opcode(OpCode::OpConst, scanner.get_line());
    //    let lambda = parse_defun(scanner, self)?;
    //    let idx = chunk.add_constant(Value::Obj(Box::new(lambda.clone())));
    //    chunk.write_constant(idx as u8, scanner.get_line());
    //    let idx = self.set_local(dbg!(
    //        lambda.get_function().ok_or("Failed to find function")?.name
    //    ));
    //    chunk.write_opcode(OpCode::OpSetLocal, scanner.get_line());
    //    chunk.write_constant(idx as u8, scanner.get_line());

    //    println!("defun: {}", chunk);
    //    println!("function: {}", lambda.get_function().unwrap().chunk);

    //    Ok(())
    //}

    fn emit_lambda(&mut self, chunk: &mut Chunk, scanner: &mut Scanner) -> Result<(), String> {
        chunk.write_opcode(OpCode::OpClosure, scanner.get_line());
        let (lambda, lambda_compiler) = parse_lambda(scanner, self)?;
        let idx = chunk.add_constant(Value::Obj(Box::new(lambda.clone())));
        chunk.write_constant(idx as u8, scanner.get_line());

        for upval in lambda_compiler.upvals.iter() {
            chunk.write_constant(upval.is_local as u8, scanner.get_line());
            chunk.write_constant(upval.index as u8, scanner.get_line());
        }

        Ok(())
    }

    fn emit_integer(&self, chunk: &mut Chunk, atom: (Token, usize)) -> Result<(), String> {
        let i: i32 = atom.0.atom()?.parse().unwrap();
        chunk.write_opcode(OpCode::OpConst, atom.1);
        let constant = chunk.add_constant(Value::Number(i as f64));
        chunk.write_constant(constant as u8, atom.1);
        Ok(())
    }
    fn emit_string(&self, chunk: &mut Chunk, atom: (Token, usize)) -> Result<(), String> {
        chunk.write_opcode(OpCode::OpConst, atom.1);
        let s: &str = &atom.0.atom()?;
        let s = Object::Str(unescape_str(&s[1..s.len() - 1]));
        let constant = chunk.add_constant(Value::Obj(Box::new(s)));
        chunk.write_constant(constant as u8, atom.1);
        Ok(())
    }

    fn emit_get_local(&self, chunk: &mut Chunk, id: usize) -> Result<(), String> {
        chunk.write_opcode(OpCode::OpGetLocal, 1);
        chunk.write_constant(id as u8, 1);
        Ok(())
    }

    fn emit_get_upvalue(&mut self, chunk: &mut Chunk, atom: &(Token, usize)) -> Result<(), String> {
        let id = self.get_upvalue(&atom.0.atom()?).unwrap();
        self.add_upvalue(id, true);
        chunk.write_opcode(OpCode::OpGetUpvalue, atom.1);
        chunk.write_constant(dbg!(id as u8), atom.1);
        Ok(())
    }

    fn emit_function_call(
        &mut self,
        chunk: &mut Chunk,
        atom: (Token, usize),
        scanner: &mut Scanner,
    ) -> Result<(), String> {
        scanner.scan().unwrap();
        chunk.write_opcode(OpCode::OpGetLocal, atom.1);
        let idx = match self.get_local(&atom.0.atom()?) {
            Some(idx) => idx,
            None => {
                return Err(format!("Symbol {} is not defined", atom.0.atom()?));
            }
        };
        chunk.write_constant(idx as u8, 1);
        loop {
            if scanner.peek().unwrap().0 == Token::RightParen {
                break;
            }
            parse(scanner, chunk, self)?;
        }

        chunk.write_opcode(OpCode::OpCall, atom.1);
        Ok(())
    }

    fn resolve_variable(&mut self, chunk: &mut Chunk, atom: &(Token, usize)) -> Result<(), String> {
        let local = self.get_local(&atom.0.atom()?);
        if local.is_some() {
            self.emit_get_local(chunk, local.unwrap())?;
        } else {
            self.emit_get_upvalue(chunk, atom)?;
        }
        Ok(())
    }
}

pub fn compile(source: &str, chunk: &mut Chunk, compiler: &mut Compiler) -> Result<(), String> {
    let mut scanner = Scanner::new(source);
    parse(&mut scanner, chunk, compiler)?;
    chunk.write_opcode(OpCode::OpRet, scanner.get_line());

    Ok(())
}

fn parse(scanner: &mut Scanner, chunk: &mut Chunk, compiler: &mut Compiler) -> Result<(), String> {
    let token = match scanner.peek() {
        Some(x) => x,
        None => return Ok(()),
    };

    match &token.0 {
        Token::LeftParen => read_seq(scanner, chunk, compiler)?,
        Token::RightParen => {
            return Err("unexpected ')'".to_string());
        }
        Token::Atom(_) => {
            scanner.scan().unwrap();
            read_atom(token, scanner, chunk, compiler)?;
        }
    };

    Ok(())
}

fn unary(
    op: (Token, usize),
    scanner: &mut Scanner,
    chunk: &mut Chunk,
    compiler: &mut Compiler,
) -> Result<(), String> {
    parse(scanner, chunk, compiler)?;
    match &op.0.atom()? as &str {
        "not" => chunk.write_opcode(OpCode::OpNot, op.1),
        _ => {
            return Err(format!("Unexpected unary operation {}", op.0.atom()?));
        }
    };

    Ok(())
}

fn binary(
    op: (Token, usize),
    scanner: &mut Scanner,
    chunk: &mut Chunk,
    compiler: &mut Compiler,
) -> Result<(), String> {
    scanner.scan().unwrap();
    parse(scanner, chunk, compiler)?;
    parse(scanner, chunk, compiler)?;

    match &op.0.atom()? as &str {
        "+" => chunk.write_opcode(OpCode::OpAdd, op.1),
        "-" => chunk.write_opcode(OpCode::OpSub, op.1),
        "*" => chunk.write_opcode(OpCode::OpMul, op.1),
        "/" => chunk.write_opcode(OpCode::OpDiv, op.1),
        "=" => chunk.write_opcode(OpCode::OpEq, op.1),
        "!=" => chunk.write_opcode(OpCode::OpNe, op.1),
        ">" => chunk.write_opcode(OpCode::OpBt, op.1),
        ">=" => chunk.write_opcode(OpCode::OpBe, op.1),
        "<" => chunk.write_opcode(OpCode::OpLt, op.1),
        "<=" => chunk.write_opcode(OpCode::OpLe, op.1),
        "and" => chunk.write_opcode(OpCode::OpAnd, op.1),
        "nand" => chunk.write_opcode(OpCode::OpNand, op.1),
        "or" => chunk.write_opcode(OpCode::OpOr, op.1),
        "nor" => chunk.write_opcode(OpCode::OpNor, op.1),
        "xor" => chunk.write_opcode(OpCode::OpXor, op.1),
        "xnor" => chunk.write_opcode(OpCode::OpXnor, op.1),
        _ => return Err(format!("Unexpected binary operation: {}", op.0.atom()?)),
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
    match op.0 {
        Token::Atom(_) => read_atom(op, scanner, chunk, compiler)?,
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
    assert!(scanner.scan().unwrap().0 == Token::LeftParen);
    let mut v: Vec<Token> = Vec::new();
    loop {
        let s = scanner.scan().unwrap().0;
        if s == Token::RightParen {
            break;
        }

        v.push(s);
    }

    Some(v)
}

fn parse_lambda(
    scanner: &mut Scanner,
    compiler: &mut Compiler,
) -> Result<(Object, Compiler), String> {
    assert!(scanner.scan().unwrap().0 == Token::Atom("lambda".to_string()));
    let args = read_shallow_list(scanner).unwrap();
    let mut rng = rand::thread_rng();
    let r: u32 = rng.gen();
    let name = format!("f{}", r);

    let mut compiler = Compiler::new(
        Some(Box::new((*compiler).clone())),
        Ctx::FunctionScope(String::from("lambda")),
    );

    let mut function = Function {
        arity: args.len(),
        chunk: Chunk::new(&name),
        name,
        upvalue_count: 0,
    };

    for arg in args {
        compiler.set_local(arg.atom()?);
    }
    parse(scanner, &mut function.chunk, &mut compiler)?;

    function.chunk.write_opcode(OpCode::OpRet, 1);
    function.upvalue_count = compiler.upvals.len();
    Ok((Object::Function(Box::new(function)), compiler))
}

//fn parse_defun(scanner: &mut Scanner, compiler: &mut Compiler) -> Result<Object, String> {
//    assert!(scanner.scan().unwrap().0 == Token::Atom("defun".to_string()));
//    let name = scanner.scan().unwrap().0.atom().unwrap();
//    let args = read_shallow_list(scanner).unwrap();
//
//    let mut compiler = Compiler::new(
//        Some(Box::new((*compiler).clone())),
//        Ctx::FunctionScope(name.clone()),
//    );
//
//    let mut closure = Closure {
//        params: args
//            .iter()
//            .map(|x| x.atom().unwrap())
//            .collect::<Vec<String>>(),
//        chunk: Chunk::new(&name),
//        name: name.clone(),
//        upvalues: Vec::new(),
//    };
//
//    for arg in args {
//        compiler.set_local(arg.atom()?);
//    }
//    parse(scanner, &mut closure.chunk, &mut compiler)?;
//    closure.chunk.write_opcode(OpCode::OpRet, 1);
//
//    Ok(Object::Function(Box::new(closure)))
//}

fn read_atom(
    atom: (Token, usize),
    scanner: &mut Scanner,
    chunk: &mut Chunk,
    compiler: &mut Compiler,
) -> Result<(), String> {
    lazy_static! {
        static ref INT_RE: Regex = Regex::new(r"^-?[0-9]+$").unwrap();
        static ref STR_RE: Regex = Regex::new(r#""(?:\\.|[^\\"])*""#).unwrap();
    }

    match &atom.0.atom()? as &str {
        "nil" => compiler.emit_nil(chunk, atom.1),
        "true" => compiler.emit_true(chunk, atom.1),
        "false" => compiler.emit_false(chunk, atom.1),
        "+" | "-" | "*" | "/" | "=" | "!=" | "<" | "<=" | ">" | ">=" | "and" | "nand" | "or"
        | "nor" | "xor" | "xnor" => compiler.emit_binary_operation(chunk, atom, scanner),
        "print" => compiler.emit_print(chunk, atom, scanner),
        "set!" => {
            scanner.scan().unwrap();
            let name = scanner.scan().unwrap();
            parse(scanner, chunk, compiler)?;
            if compiler.get_local(&name.0.atom()?).is_some() {
                compiler.emit_set_local(chunk, &name)
            } else if compiler.get_upvalue(&name.0.atom()?).is_some() {
                compiler.emit_set_upvalue(chunk, &name.0.atom()?)
            } else {
                compiler.emit_set_local(chunk, &name)
            }
        }
        "if" => compiler.emit_if(chunk, scanner),
        "not" => compiler.emit_not(chunk, atom, scanner),
        "do" => compiler.emit_do(chunk, scanner),
        "lambda" => compiler.emit_lambda(chunk, scanner),
        //"defun" => compiler.emit_defun(chunk, scanner),
        _ => {
            if INT_RE.is_match(&atom.0.atom()?) {
                compiler.emit_integer(chunk, atom)
            } else if STR_RE.is_match(&atom.0.atom()?) {
                compiler.emit_string(chunk, atom)
            } else if scanner.previous() != Some(Token::LeftParen) {
                //resolve variable
                compiler.resolve_variable(chunk, &atom)
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
        Compiler::new(None, Ctx::TopLevel)
    }

    #[fixture]
    fn chunk() -> Chunk {
        Chunk::new("main")
    }

    #[rstest]
    fn test_locals(mut compiler: Compiler) {
        compiler.set_local(String::from("x"));
        compiler.set_local(String::from("y"));
        assert_eq!(compiler.get_local("x"), Some(0));
        assert_eq!(compiler.get_local("y"), Some(1));
    }

    #[rstest]
    fn test_emit_nil(compiler: Compiler, mut chunk: Chunk) {
        compiler.emit_nil(&mut chunk, 1).unwrap();
        assert_eq!(chunk.get_code(), vec![op!(OpCode::OpNil)]);
    }

    #[rstest]
    fn test_emit_true(compiler: Compiler, mut chunk: Chunk) {
        compiler.emit_true(&mut chunk, 1).unwrap();
        assert_eq!(chunk.get_code(), vec![op!(OpCode::OpTrue)]);
    }

    #[rstest]
    fn test_emit_false(compiler: Compiler, mut chunk: Chunk) {
        compiler.emit_false(&mut chunk, 1).unwrap();
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
