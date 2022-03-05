use std::cmp::Ordering;
use std::collections::HashMap;
use std::fmt;
use std::fmt::Write as _;

pub mod closure;
pub mod object;
pub mod value;
use closure::Closure;
pub use value::Value;

#[derive(Debug, Clone, Copy, PartialEq)]
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

#[derive(Debug, Clone, PartialEq)]
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

    fn get_opcode(&self) -> Option<OpCode> {
        match self {
            Element::OpCode(op) => Some(*op),
            _ => None,
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct Chunk {
    name: String,
    code: Vec<Element>,
    pub constants: Vec<Value>,
    lines: Vec<(usize, usize)>,
    //functions: HashMap<String, Closure>,
}

impl fmt::Display for Chunk {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        //writeln!(f, "{:?}", self.constants)?;
        //writeln!(f, "{:?}", self.code)?;
        writeln!(f, "==={}===", &self.name)?;
        let mut pc: usize = 0;
        loop {
            let (s, inc) = match self.display_instruction(pc) {
                Some((s, inc)) => (s, inc),
                None => return Ok(()),
            };
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

    pub fn get_current_index(&self) -> Result<usize, String> {
        if self.code.len() >= 1 {
            Ok(self.code.len() - 1)
        } else {
            Err("get_current_index: no code to be found".to_string())
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

    pub fn rewrite_constant(&mut self, idx: usize, constant: u8) {
        self.code[idx] = Element::Constant(constant);
    }

    pub fn write_constant_long(&mut self, constant: usize, line: usize) {
        self.write(Element::Constant((constant >> 16 & 0xFF) as u8), line);
        self.write(Element::Constant((constant >> 8 & 0xFF) as u8), line);
        self.write(Element::Constant((constant & 0xFF) as u8), line);
    }

    pub fn len(&self) -> usize {
        self.code.len()
    }

    pub fn is_ip_in_range(&self, ip: usize) -> bool {
        self.code.len() > ip
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

    pub fn get_opcode(&self, index: usize) -> Option<&OpCode> {
        let op = self.code.get(index)?;

        match &op {
            Element::OpCode(opcode) => Some(opcode),
            _ => {
                println!("Expected opcode got: {:?}", self.code[index]);
                None
            }
        }
    }

    pub fn get_constant_index(&self, index: usize) -> usize {
        match &self.code[index] {
            Element::Constant(i) => *i as usize,
            _ => {
                println!("Expected Constant got: {:?}", self.code[index]);
                panic!();
            }
        }
    }

    pub fn get_constant(&self, index: usize) -> (usize, &Value) {
        let idx: usize = self.get_constant_index(index);
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

#[cfg(test)]
mod tests {
    use super::*;

    fn fixture_closure() -> Closure {
        Closure {
            params: vec!["x".to_string(), "y".to_string()],
            chunk: Chunk::new("test_chunk"),
            name: "test_closure".to_string(),
        }
    }

    #[test]
    fn test_element_get_constant() {
        assert_eq!(Element::Constant(1).get_constant(), Some(1));
        assert_eq!(Element::OpCode(OpCode::OpNil).get_constant(), None);
    }

    #[test]
    fn test_element_get_opcode() {
        assert_eq!(Element::Constant(1).get_opcode(), None);
        assert_eq!(
            Element::OpCode(OpCode::OpNil).get_opcode(),
            Some(OpCode::OpNil)
        );
    }

    #[test]
    fn test_chunk_get_name() {
        let chunk = Chunk::new("test_chunk");
        assert_eq!(chunk.get_name(), "test_chunk");
    }

    #[test]
    fn test_chunk_get_code() {
        let mut chunk = Chunk::new("test_chunk");
        assert_eq!(chunk.get_code(), vec![]);
    }

    #[test]
    fn test_chunk_get_current_index() {
        let mut chunk = Chunk::new("test_chunk");
        assert_eq!(chunk.get_current_index().ok(), None);
    }

    #[test]
    fn test_chunk_write() {
        let mut chunk = Chunk::new("test_chunk");
        chunk.write(Element::OpCode(OpCode::OpNil), 0);
        assert_eq!(chunk.get_current_index().ok(), Some(0));
        chunk.write(Element::Constant(0), 0);
        assert_eq!(chunk.get_current_index().ok(), Some(1));
    }

    #[test]
    fn test_chunk_write_opcode() {
        let mut chunk = Chunk::new("test_chunk");
        chunk.write_opcode(OpCode::OpNil, 0);
        assert_eq!(chunk.get_current_index().ok(), Some(0));
        chunk.write_opcode(OpCode::OpNil, 0);
        assert_eq!(chunk.get_current_index().ok(), Some(1));
    }

    #[test]
    fn test_chunk_write_constant() {
        let mut chunk = Chunk::new("test_chunk");
        chunk.write_constant(0, 0);
        assert_eq!(chunk.get_current_index().ok(), Some(0));
        chunk.write_constant(1, 0);
        assert_eq!(chunk.get_current_index().ok(), Some(1));
    }

    #[test]
    fn test_chunk_rewrite_constant() {
        let mut chunk = Chunk::new("test_chunk");
        let idx = chunk.add_constant(Value::Number(2.0));
        chunk.write_constant(idx as u8, 0);
        assert_eq!(chunk.get_current_index().ok(), Some(0));

        chunk.rewrite_constant(idx, 1);
        assert_eq!(chunk.get_constant_index(idx), 1);
    }

    #[test]
    fn test_chunk_write_constant_long() {
        let mut chunk = Chunk::new("test_chunk");
        chunk.write_constant_long(0, 0);
        assert_eq!(chunk.get_current_index().ok(), Some(2));
        chunk.write_constant_long(1, 0);
        assert_eq!(chunk.get_current_index().ok(), Some(5));
    }

    #[test]
    fn test_chunk_display() {
        let mut chunk = Chunk::new("test_chunk");
        assert_eq!(format!("{}", chunk), "===test_chunk===\n");

        chunk.write_opcode(OpCode::OpNil, 0);
        assert_eq!(
            format!("{}", chunk),
            "===test_chunk===\n0000 0 OpNil\n================"
        );
    }

    #[test]
    fn test_is_ip_in_range() {
        let mut chunk = Chunk::new("test_chunk");
        assert_eq!(chunk.is_ip_in_range(1), false);
    }

    #[test]
    fn test_annotate_line() {
        let mut chunk = Chunk::new("test_chunk");
        chunk.annotate_line(1);
        chunk.annotate_line(0);
        chunk.annotate_line(0);

        assert_eq!(*chunk.get_line(0), 1_usize);
        assert_eq!(*chunk.get_line(1), 0_usize);
        assert_eq!(*chunk.get_line(2), 0_usize);
    }
}
