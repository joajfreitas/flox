use crate::chunk::{Chunk, OpCode, Value};

const DEBUG: bool = true;

pub struct VirtualMachine {
    ip: usize,
    stack: Vec<Value>
}

pub enum VMErr {
    InterpretCompileError,
    InterpretRuntimeError,
}

impl VirtualMachine {
    pub fn new() -> VirtualMachine {
        VirtualMachine { 
            ip: 0,
            stack: Vec::new()
        }
    }

    pub fn interpret(&mut self, chunk: Chunk) -> Result<(), VMErr> {
        self.run(chunk)
    }

    fn run(&mut self, chunk: Chunk) -> Result<(), VMErr> {
        loop {
            if DEBUG == true {
                let (s, _) = chunk.display_instruction(self.ip);
                print!("{}", s);
                println!("stack: {:?}", self.stack);
            }
            let opcode = chunk.get_opcode(self.ip);
            match opcode {
                OpCode::OpReturn => return Ok(()),
                OpCode::OpConstant => {
                    let (_, value) = chunk.get_constant(self.ip+1);
                    self.stack.push(*value);
                    self.ip+=2
                },
                OpCode::OpConstantLong => {
                    let value = chunk.get_constant_long(self.ip+1).unwrap(); 
                    self.stack.push(*value);
                    self.ip+=4
                },
                OpCode::OpNegate => {
                    let v = self.stack.pop().unwrap().get_value();
                    self.stack.push(Value::Value(-v));
                    self.ip+=1
                },
                OpCode::OpAdd => {
                    let op2 = self.stack.pop().unwrap().get_value();
                    let op1 = self.stack.pop().unwrap().get_value();
                    self.stack.push(Value::Value(op1 + op2));
                    self.ip+=1
                },
                OpCode::OpSubtract => {
                    let op2 = self.stack.pop().unwrap().get_value();
                    let op1 = self.stack.pop().unwrap().get_value();
                    self.stack.push(Value::Value(op1 - op2));
                    self.ip+=1
                },
                OpCode::OpMultiply => {
                    let op2 = self.stack.pop().unwrap().get_value();
                    let op1 = self.stack.pop().unwrap().get_value();
                    self.stack.push(Value::Value(op1 * op2));
                    self.ip+=1
                },
                OpCode::OpDivide => {
                    let op2 = self.stack.pop().unwrap().get_value();
                    let op1 = self.stack.pop().unwrap().get_value();
                    self.stack.push(Value::Value(op1 / op2));
                    self.ip+=1
                },
            }
        }
    }
}
