use std::fmt;
use std::cmp::Ordering;
use std::ops::{Not, Add, Sub, Mul, Div, BitAnd, BitOr, BitXor};

#[derive(Debug)]
pub enum OpCode {
    OpReturn,
    OpConstant,
    OpConstantLong,
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
}

#[derive(Debug, Copy, Clone)]
pub enum Value {
    Number(f64),
    Bool(bool),
    Nil,
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

    pub fn is_nil(&self) -> bool {
        match self {
            Value::Nil => true,
            _ => false,
        }
    }

    pub fn is_number(&self) -> bool {
        match self {
            Value::Number(_) => true,
            _ => false,
        }
    }

    pub fn is_bool(&self) -> bool {
        match self {
            Value::Bool(_) => true,
            _ => false,
        }
    }
}

impl PartialEq for Value {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (Value::Bool(b1), Value::Bool(b2)) => b1 == b2,
            _ => panic!(),
        }
    }

    fn ne(&self, other: &Self) -> bool {
        !self.eq(other)
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
        match (self, other) {
            (Value::Number(n1), Value::Number(n2)) => Value::Number(n1 + n2),
            _ => panic!(),
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
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Value::Number(value) => write!(f, "{:.1}", value),
            Value::Bool(value) => write!(f, "{:1}", value),
            Value::Nil => write!(f, "nil"),
        }
    }
}

#[derive(Debug)]
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

pub struct Chunk<'a> {
    name: &'a str,
    code: Vec<Element>,
    constants: Vec<Value>,
    lines: Vec<(usize, usize)>,
}

impl fmt::Display for Chunk<'_> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "==={}===\n", &self.name)?;
        let mut pc: usize = 0;
        loop {
            let (s, inc) = self.display_instruction(pc);
            pc += inc;
            write!(f, "{}", s)?;
            if pc >= self.code.len() {
                break;
            }
        }

        //write!(f, "{:?}", self.lines)?;
        write!(f, "================")?;

        Ok(())
    }
}

impl Chunk<'_> {
    pub fn new<'a>(name: &'a str) -> Chunk {
        Chunk {
            name: name,
            code: Vec::new(),
            constants: Vec::new(),
            lines: Vec::new(),
        }
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

    pub fn write_constant_long(&mut self, constant: usize, line: usize) {
        self.write
            (Element::Constant((constant >> 16 & 0xFF) as u8), line);
        self.write
            (Element::Constant((constant >> 8 & 0xFF) as u8), line);
        self.write
            (Element::Constant((constant >> 0 & 0xFF) as u8), line);
    }

    fn annotate_line(&mut self, line: usize) {
        if self.lines.len() == 0 {
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

        return &0;
    }

    pub fn add_constant(&mut self, value: Value) -> usize {
        self.constants.push(value);
        self.constants.len() - 1
    }

    pub fn get_opcode(&self, index: usize) -> &OpCode {
        match &self.code[index] {
            Element::OpCode(opcode) => opcode,
            _ => panic!(),
        }
    }

    pub fn get_constant(&self, index: usize) -> (usize, &Value) {
        match &self.code[index] {
            Element::Constant(i) => (*i as usize, &self.constants[*i as usize]),
            _ => panic!(),
        }
    }

    pub fn get_constant_long(&self, index: usize) -> Option<&Value> {
        let c1 = self.code[index].get_constant()?;
        let c2 = self.code[index + 1].get_constant()?;
        let c3 = self.code[index + 2].get_constant()?;
        let index: usize = ((c1 as usize) << 16) + ((c2 as usize) << 8) + c3 as usize;
        Some(&self.constants[index as usize])
    }

    pub fn display_instruction(
        &self,
        index: usize,
    ) -> (String, usize) {

        let mut s = String::new();

        s.push_str(&format!("{:0>4} ", index));

        if index > 0 && self.get_line(index) == self.get_line(index - 1) {
            s.push_str(&format!(
                "{}| ",
                std::iter::repeat(" ")
                    .take(self.get_line(index).to_string().chars().count())
                    .collect::<String>()
            ));
        } else {
            s.push_str(&format!("{} ", self.get_line(index)));
        }


        let opcode = self.get_opcode(index);
        let (ss, i) = match opcode {
            OpCode::OpConstant => {
                let (n, c) = self.get_constant(index + 1);
                (format!("{:?} {}:'{}'\n", opcode, n, c), 2)
            }
            OpCode::OpConstantLong => {
                let value = self.get_constant_long(index + 1).unwrap();
                (format!("{:?} '{}'\n", opcode, value), 4)
            },
            _ => (format!("{:?}\n", opcode), 1),
        };

        s.push_str(&ss);
        (s, i)
    }
}
