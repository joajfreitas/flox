use std::cmp::Ordering;
use std::fmt::Write as _;
//use std::collections::HashMap;
use std::fmt;
use std::ops::{Add, BitAnd, BitOr, BitXor, Div, Mul, Not, Sub};

#[derive(Debug, Clone)]
pub enum OpCode {
    OpReturn,
    OpConstant,
    OpConstantLong,
    OpSetLocal,
    OpGetLocal,
    OpAdd,
    OpSubtract,
    OpMultiply,
    OpDivide,
    OpNil,
    OpTrue,
    OpFalse,
    OpNot,
    OpEq,
    OpNe,
    OpBt,
    OpLt,
    OpBe,
    OpLe,
    OpAnd,
    OpNand,
    OpOr,
    OpNor,
    OpXor,
    OpXnor,
    OpJmpIfFalse,
    OpJmp,
    OpCall,
}

#[derive(Clone)]
pub struct Closure {
    pub params: Vec<String>,
    pub chunk: Chunk,
    pub name: String,
}

impl fmt::Debug for Closure {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "closure")
    }
}

#[derive(Debug, Clone)]
pub enum Object {
    Str(String),
    Function(Box<Closure>),
}

impl Object {
    fn get_str(&self) -> &str {
        match self {
            Object::Str(s) => s,
            _ => panic!(),
        }
    }

    fn get_function(&self) -> Box<Closure> {
        match self {
            Object::Function(f) => f.clone(),
            _ => panic!(),
        }
    }

    fn is_function(&self) -> bool {
        matches!(self, Object::Function(_))
    }
}

#[derive(Debug, Clone)]
pub enum Value {
    Number(f64),
    Bool(bool),
    Nil,
    Obj(Box<Object>),
}

impl Value {
    pub fn get_number(&self) -> f64 {
        match self {
            Value::Number(f) => *f,
            _ => panic!(),
        }
    }

    pub fn get_bool(&self) -> bool {
        match self {
            Value::Bool(b) => *b,
            _ => panic!(),
        }
    }

    pub fn get_str(&self) -> &str {
        match self {
            Value::Obj(obj) => obj.get_str(),
            _ => {
                panic!()
            }
        }
    }

    pub fn get_function(&self) -> Box<Closure> {
        match self {
            Value::Obj(obj) => obj.get_function(),
            _ => panic!(),
        }
    }

    pub fn is_nil(&self) -> bool {
        matches!(self, Value::Nil)
    }

    pub fn is_number(&self) -> bool {
        matches!(self, Value::Number(_))
    }

    pub fn is_bool(&self) -> bool {
        matches!(self, Value::Bool(_))
    }

    pub fn is_function(&self) -> bool {
        match self {
            Value::Obj(f) => f.is_function(),
            _ => false,
        }
    }
}

impl PartialEq for Value {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (Value::Bool(b1), Value::Bool(b2)) => b1 == b2,
            (Value::Number(b1), Value::Number(b2)) => b1 == b2,
            _ => panic!(),
        }
    }
}

impl Eq for Value {}

impl PartialOrd for Value {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        match (self, other) {
            (Value::Bool(b1), Value::Bool(b2)) => Some(b1.cmp(b2)),
            (Value::Number(n1), Value::Number(n2)) => n1.partial_cmp(n2),
            _ => panic!(),
        }
    }
}

impl Add for Value {
    type Output = Self;

    fn add(self, other: Self) -> Self {
        match (&self, &other) {
            (Value::Number(n1), Value::Number(n2)) => Value::Number(n1 + n2),
            _ => {
                println!("add: {} {}", &self, &other);
                panic!();
            }
        }
    }
}

impl Sub for Value {
    type Output = Self;

    fn sub(self, other: Self) -> Self {
        match (self, other) {
            (Value::Number(n1), Value::Number(n2)) => Value::Number(n1 - n2),
            _ => panic!(),
        }
    }
}

impl Mul for Value {
    type Output = Self;

    fn mul(self, other: Self) -> Self {
        match (self, other) {
            (Value::Number(n1), Value::Number(n2)) => Value::Number(n1 * n2),
            _ => panic!(),
        }
    }
}

impl Div for Value {
    type Output = Self;

    fn div(self, other: Self) -> Self {
        match (self, other) {
            (Value::Number(n1), Value::Number(n2)) => Value::Number(n1 / n2),
            _ => panic!(),
        }
    }
}

impl BitAnd for Value {
    type Output = Self;
    fn bitand(self, rhs: Self) -> Self {
        match (self, rhs) {
            (Value::Bool(b1), Value::Bool(b2)) => Value::Bool(b1 & b2),
            _ => panic!(),
        }
    }
}

impl BitOr for Value {
    type Output = Self;
    fn bitor(self, rhs: Self) -> Self {
        match (self, rhs) {
            (Value::Bool(b1), Value::Bool(b2)) => Value::Bool(b1 | b2),
            _ => panic!(),
        }
    }
}

impl BitXor for Value {
    type Output = Self;
    fn bitxor(self, rhs: Self) -> Self {
        match (self, rhs) {
            (Value::Bool(b1), Value::Bool(b2)) => Value::Bool(b1 ^ b2),
            _ => panic!(),
        }
    }
}

impl Not for Value {
    type Output = Self;

    fn not(self) -> Self::Output {
        match self {
            Value::Bool(b) => Value::Bool(!b),
            _ => panic!(),
        }
    }
}

impl fmt::Display for Value {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Value::Number(value) => write!(f, "{}", value),
            Value::Bool(value) => write!(f, "{:1}", value),
            Value::Nil => write!(f, "nil"),
            Value::Obj(obj) => match &**obj {
                Object::Str(s) => write!(f, "{:1}", s),
                Object::Function(_) => write!(f, "function"),
            },
        }
    }
}

#[derive(Debug, Clone)]
pub enum Element {
    OpCode(OpCode),
    Constant(u8),
}

impl Element {
    fn get_constant(&self) -> Option<u8> {
        match self {
            Element::Constant(i) => Some(*i),
            _ => None,
        }
    }
}

#[derive(Debug, Clone)]
pub struct Chunk {
    name: String,
    code: Vec<Element>,
    pub constants: Vec<Value>,
    lines: Vec<(usize, usize)>,
    //functions: HashMap<String, Closure>,
}

impl fmt::Display for Chunk {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        writeln!(f, "{:?}", self.constants)?;
        writeln!(f, "{:?}", self.code)?;
        writeln!(f, "==={}===", &self.name)?;
        let mut pc: usize = 0;
        loop {
            let (s, inc) = self.display_instruction(pc).unwrap();
            pc += inc;
            write!(f, "{}", s)?;
            if pc >= self.code.len() {
                break;
            }
        }

        write!(f, "================")?;

        Ok(())
    }
}

impl Chunk {
    pub fn new(name: &str) -> Chunk {
        Chunk {
            name: name.to_string(),
            code: Vec::new(),
            constants: Vec::new(),
            lines: Vec::new(),
            //functions: HashMap::new(),
        }
    }

    pub fn get_name(&self) -> String {
        self.name.clone()
    }

    pub fn get_code(&mut self) -> Vec<Element> {
        self.code.clone()
    }

    pub fn get_current_index(&self) -> usize {
        self.code.len() - 1
    }

    pub fn write(&mut self, element: Element, line: usize) {
        self.code.push(element);
        self.annotate_line(line)
    }

    pub fn write_opcode(&mut self, opcode: OpCode, line: usize) {
        self.write(Element::OpCode(opcode), line);
    }

    pub fn write_constant(&mut self, constant: u8, line: usize) {
        self.write(Element::Constant(constant), line);
    }

    pub fn rewrite_constant(&mut self, idx: usize, constant: u8) {
        self.code[idx] = Element::Constant(constant);
    }

    pub fn write_constant_long(&mut self, constant: usize, line: usize) {
        self.write(Element::Constant((constant >> 16 & 0xFF) as u8), line);
        self.write(Element::Constant((constant >> 8 & 0xFF) as u8), line);
        self.write(Element::Constant((constant & 0xFF) as u8), line);
    }

    fn annotate_line(&mut self, line: usize) {
        if self.lines.is_empty() {
            self.lines.push((line, 1));
        } else {
            let l = self.lines.len() - 1;
            let r = self.lines[l];
            if r.0 == line {
                self.lines[l] = (r.0, r.1 + 1);
            } else {
                self.lines.push((line, 1));
            }
        }
    }

    fn get_line(&self, index: usize) -> &usize {
        let mut acc = 0;
        for (line, count) in &self.lines {
            acc += count;
            if index < acc {
                return line;
            }
        }

        &0
    }

    pub fn add_constant(&mut self, value: Value) -> usize {
        self.constants.push(value);
        self.constants.len() - 1
    }

    pub fn len(&self) -> usize {
        self.code.len()
    }

    pub fn is_empty(&self) -> bool {
        self.code.len() == 0
    }

    pub fn is_ip_in_range(&self, ip: usize) -> bool {
        self.code.len() > ip
    }

    pub fn get_opcode(&self, index: usize) -> Option<&OpCode> {
        let op = self.code.get(index)?;

        match &op {
            Element::OpCode(opcode) => Some(opcode),
            _ => {
                println!("Expected opcode got: {:?}", self.code[index]);
                panic!();
            }
        }
    }

    pub fn get_constant_index(&self, index: usize) -> u8 {
        match &self.code[index] {
            Element::Constant(i) => *i,
            _ => {
                println!("Expected Constant got: {:?}", self.code[index]);
                panic!();
            }
        }
    }

    pub fn get_constant(&self, index: usize) -> (usize, &Value) {
        let idx: usize = self.get_constant_index(index) as usize;
        (idx, &self.constants[idx])
    }

    pub fn get_constant_long(&self, index: usize) -> Option<&Value> {
        let c1 = self.code[index].get_constant()?;
        let c2 = self.code[index + 1].get_constant()?;
        let c3 = self.code[index + 2].get_constant()?;
        let index: usize = ((c1 as usize) << 16) + ((c2 as usize) << 8) + c3 as usize;
        Some(&self.constants[index as usize])
    }

    pub fn display_instruction(&self, index: usize) -> Option<(String, usize)> {
        let mut s = String::new();

        write!(s, "{:0>4} ", index).unwrap();

        if index > 0 && self.get_line(index) == self.get_line(index - 1) {
            write!(
                s,
                "{}| ",
                " ".repeat(self.get_line(index).to_string().chars().count())
            )
            .unwrap();
        } else {
            write!(s, "{}", self.get_line(index)).unwrap();
        }

        let opcode = self.get_opcode(index)?;
        let (ss, i) = match opcode {
            OpCode::OpConstant => {
                let (n, c) = self.get_constant(index + 1);
                (format!("{:?} {}:'{}'\n", opcode, n, c), 2)
            }
            OpCode::OpConstantLong => {
                let value = self.get_constant_long(index + 1).unwrap();
                (format!("{:?} '{}'\n", opcode, value), 4)
            }
            OpCode::OpSetLocal => {
                let (n, c) = self.get_constant(index + 1);
                (format!("{:?} {}: '{}\n", opcode, n, c), 2)
            }
            OpCode::OpGetLocal => {
                let n = self.get_constant_index(index + 1);
                (format!("{:?} {}\n", opcode, n), 2)
            }
            OpCode::OpJmpIfFalse | OpCode::OpJmp => {
                let idx = self.get_constant_index(index + 1);
                (format!("{:?}: {}\n", opcode, idx), 2)
            }
            _ => (format!("{:?}\n", opcode), 1),
        };

        s.push_str(&ss);
        Some((s, i))
    }
}
