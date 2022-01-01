use std::collections::HashMap;
use crate::chunk::{Chunk, OpCode, Value, Closure};

const DEBUG: bool = true;

struct CallFrame {
    function: Box<Closure>,
    ip: usize,
    stackpointer: usize
}

pub struct VirtualMachine {
    stack: Vec<Value>,
    frames: Vec<CallFrame>,
    fp: usize,
}

pub enum VMErr {
    CompileError,
    RuntimeError(String),
}


macro_rules! nullary {
    ($fn:expr, $self:expr, $ip:expr) => {
        {
            $self.stack.push($fn());
            $self.set_ip($ip + 1);
        }
    };
}

macro_rules! unary {
    ($fn:expr, $self:expr, $ip:expr) => {
        {
            let arg = $self.stack.pop().unwrap();
            $self.stack.push($fn(arg));
            $self.set_ip($ip + 1);
        }
    };
}

macro_rules! binary {
    ($fn:expr, $self:expr, $ip:expr) => {
        {
            let arg2 = $self.stack.pop().unwrap();
            let arg1 = $self.stack.pop().unwrap();
            $self.stack.push($fn(arg1, arg2));
            $self.set_ip($ip + 1);
        }
    };
}

impl VirtualMachine {
    pub fn new() -> VirtualMachine {
        VirtualMachine { 
            stack: Vec::new(),
            frames: Vec::new(),
            fp: 0,
        }
    }

    pub fn get_chunk(&self) -> Chunk {
        self.frames[self.fp].function.chunk.clone()
    }

    pub fn get_ip(&self) -> usize {
        self.frames[self.fp].ip
    }

    pub fn set_ip(&mut self, ip: usize) {
        self.frames[self.fp].ip = ip;
    }

    pub fn interpret(&mut self, chunk: &mut Chunk) -> Result<Value, VMErr> {
        self.run(chunk)
    }

    fn run(&mut self, chunk: &mut Chunk) -> Result<Value, VMErr> {
        let frame = CallFrame {
            function : Box::new(Closure {
                params: Vec::new(),
                chunk: chunk.clone(),
                name: "main".to_string(),
            }),
            ip: 0,
            stackpointer: 0,
        };

        self.frames.pop(); //cleanup callstack
        self.frames.push(frame);

        loop {
            let chunk = self.get_chunk();
            let ip = self.get_ip();
            if !chunk.is_ip_in_range(ip) {
                return Err(VMErr::RuntimeError(format!("Attemting to access unreachable bytecode. ip: {}, len: {}", ip, chunk.len())));
            }
            if DEBUG {
                let (s, _) = chunk.display_instruction(ip).unwrap();
                print!("{}", s);
                println!("stack: {:?}", self.stack);
            }
            let opcode = chunk.get_opcode(ip).unwrap();
            match opcode {
                OpCode::OpReturn => {
                    match self.stack.pop() {
                        Some(x) => return Ok(x),
                        None => return Ok(Value::Nil),
                    };
                }
                OpCode::OpConstant => {
                    let (_, value) = chunk.get_constant(ip+1);
                    self.stack.push(value.clone());
                    self.set_ip(ip+2);
                },
                OpCode::OpConstantLong => {
                    let value = chunk.get_constant_long(ip+1).unwrap(); 
                    self.stack.push(value.clone());
                    self.set_ip(ip+4);
                },
                OpCode::OpNil => nullary!(||{Value::Nil}, self, ip),
                OpCode::OpTrue => nullary!(||{Value::Bool(true)}, self, ip),
                OpCode::OpFalse => nullary!(||{Value::Bool(false)}, self, ip),
                OpCode::OpAdd => binary!(|x,y|{x+y}, self, ip),
                OpCode::OpSubtract => binary!(|x,y|{x-y}, self, ip),
                OpCode::OpMultiply => binary!(|x,y|{x*y}, self, ip),
                OpCode::OpDivide => binary!(|x,y|{x/y}, self, ip),
                OpCode::OpNot => unary!(|x: Value| {!x}, self, ip),
                OpCode::OpEq => binary!(|x,y|{Value::Bool(x == y)}, self, ip),
                OpCode::OpNe => binary!(|x,y|{Value::Bool(x != y)}, self, ip),
                OpCode::OpBt => binary!(|x,y|{Value::Bool(x > y)}, self, ip),
                OpCode::OpBe => binary!(|x,y|{Value::Bool(x >= y)}, self, ip),
                OpCode::OpLt => binary!(|x,y|{Value::Bool(x < y)}, self, ip),
                OpCode::OpLe => binary!(|x,y|{Value::Bool(x <= y)}, self, ip),
                OpCode::OpAnd => binary!(|x,y|{x & y}, self, ip),
                OpCode::OpNand => binary!(|x:Value,y:Value|{!(x & y)}, self, ip),
                OpCode::OpOr => binary!(|x,y|{x | y}, self, ip),
                OpCode::OpNor => binary!(|x:Value,y:Value|{!(x | y)}, self, ip),
                OpCode::OpXor => binary!(|x,y|{x ^ y}, self, ip),
                OpCode::OpXnor => binary!(|x:Value,y:Value|{!(x ^ y)}, self, ip),
                OpCode::OpSetLocal => {
                    let value = self.stack.last().unwrap();
                    let slot = chunk.get_constant_index(ip+1);
                    self.stack[slot as usize] = value.clone();
                    self.set_ip(ip+2);
                },
                OpCode::OpGetLocal => {
                    let slot = chunk.get_constant_index(ip+1);
                    self.stack.push(self.stack[slot as usize].clone());
                    self.set_ip(ip+2);
                },
                OpCode::OpJmpIfFalse => {
                    let idx = chunk.get_constant_index(ip+1);
                    let pred = self.stack.pop().unwrap();
                    if !pred.get_bool() {
                        self.set_ip(idx as usize);
                    }
                    else { 
                        self.set_ip(ip + 2);
                    }
                }
                OpCode::OpJmp => {
                    let idx = chunk.get_constant_index(ip+1);
                    self.set_ip(idx as usize);
                },
                OpCode::OpCall => {
                    loop {
                        let v = dbg!(self.stack.pop().unwrap());
                        if v.is_function() {
                            let frame = CallFrame {
                                function: v.get_function(),
                                ip: 0,
                                stackpointer: self.stack.len() - 2,
                            };
                            self.frames.push(frame);
                            self.fp += 1;
                            break
                        }
                    }
                    panic!();
                } 
            }
        }
    }
}

impl Default for VirtualMachine {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_empty() {
        let mut vm = VirtualMachine::new();
        let mut chunk = Chunk::new("test");
        vm.interpret(&mut chunk);
    }

    #[test]
    fn test_basic() {
        let mut vm = VirtualMachine::new();
        let mut chunk = Chunk::new("test");
        chunk.write_opcode(OpCode::OpReturn, 1);
        vm.interpret(&mut chunk);

    }
}
