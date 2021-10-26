use std::fmt;

#[derive(Debug)]
pub enum OpCode {
    OpReturn,
    OpConstant,
}

pub enum Value {
    Value(f64),
}

pub struct Chunk<'a> {
    name: &'a str,
    code: Vec<OpCode>,
    constants: Vec<Value>
}

impl fmt::Display for Chunk<'_> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "==={}===", &self.name)?;
        for (offset, opcode) in self.code.iter().enumerate() {
            write!(f, "\n{:0<4} {:?}", offset, opcode)?;
        }
        Ok(())
    }
}

impl Chunk<'_> {
    pub fn new<'a>(name: &'a str) -> Chunk {
        Chunk {
            name: name,
            code : Vec::new(),
            constants: Vec::new(),
        }
    }

    pub fn write(&mut self, opcode: OpCode) {
        self.code.push(opcode);
    }

    pub fn addConstant(&mut self, value: Value) {
        self.constants.push(value);
    }
}
