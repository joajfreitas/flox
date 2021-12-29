use std::collections::HashMap;
use crate::chunk::{Chunk, OpCode, Value};

const DEBUG: bool = false;

pub struct VirtualMachine {
    ip: usize,
    stack: Vec<Value>,
    globals: HashMap<String, Value>,
    locals: HashMap<String, HashMap<String, Value>>,
}

pub enum VMErr {
    CompileError,
    RuntimeError(String),
}


macro_rules! nullary {
    ($fn:expr, $self:expr) => {
        {
            $self.stack.push($fn());
            $self.ip += 1;
        }
    };
}

macro_rules! unary {
    ($fn:expr, $self:expr) => {
        {
            let arg = $self.stack.pop().unwrap();
            $self.stack.push($fn(arg));
            $self.ip += 1;
        }
    };
}

macro_rules! binary {
    ($fn:expr, $self:expr) => {
        {
            let arg2 = $self.stack.pop().unwrap();
            let arg1 = $self.stack.pop().unwrap();
            $self.stack.push($fn(arg1, arg2));
            $self.ip += 1;
        }
    };
}

impl VirtualMachine {
    pub fn new() -> VirtualMachine {
        VirtualMachine { 
            ip: 0,
            stack: Vec::new(),
            globals: HashMap::new(),
            locals: HashMap::new(),
        }
    }

    pub fn set_local(&mut self, scope: String, key: String, value: Value) -> Option<Value>{
        if !self.locals.contains_key(&key) {
            self.locals.insert(scope.clone(), HashMap::new());
        }

        self.locals.get_mut(&scope).unwrap().insert(key, value)
    }

    pub fn get_local(&self, scope: String, key: String) -> Option<&Value> {
        self.locals.get(&scope)?.get(&key)
    }

    pub fn interpret(&mut self, chunk: &mut Chunk) -> Result<Value, VMErr> {
        self.run(chunk)
    }

    fn run(&mut self, chunk: &mut Chunk) -> Result<Value, VMErr> {
        self.ip=0;
        loop {
            if !chunk.is_ip_in_range(self.ip) {
                return Err(VMErr::RuntimeError(format!("Attemting to access unreachable bytecode. ip: {}, len: {}", self.ip, chunk.len())));
            }
            if DEBUG {
                let (s, _) = chunk.display_instruction(self.ip).unwrap();
                print!("{}", s);
                println!("stack: {:?}", self.stack);
            }
            let opcode = chunk.get_opcode(self.ip).unwrap();
            match opcode {
                OpCode::OpReturn => {
                    match self.stack.pop() {
                        Some(x) => return Ok(x),
                        None => return Ok(Value::Nil),
                    };
                }
                OpCode::OpConstant => {
                    let (_, value) = chunk.get_constant(self.ip+1);
                    self.stack.push(value.clone());
                    self.ip+=2
                },
                OpCode::OpConstantLong => {
                    let value = chunk.get_constant_long(self.ip+1).unwrap(); 
                    self.stack.push(value.clone());
                    self.ip+=4
                },
                OpCode::OpNil => nullary!(||{Value::Nil}, self),
                OpCode::OpTrue => nullary!(||{Value::Bool(true)}, self),
                OpCode::OpFalse => nullary!(||{Value::Bool(false)}, self),
                OpCode::OpAdd => binary!(|x,y|{x+y}, self),
                OpCode::OpSubtract => binary!(|x,y|{x-y}, self),
                OpCode::OpMultiply => binary!(|x,y|{x*y}, self),
                OpCode::OpDivide => binary!(|x,y|{x/y}, self),
                OpCode::OpNot => unary!(|x: Value| {!x}, self),
                OpCode::OpEq => binary!(|x,y|{Value::Bool(x == y)}, self),
                OpCode::OpNe => binary!(|x,y|{Value::Bool(x != y)}, self),
                OpCode::OpBt => binary!(|x,y|{Value::Bool(x > y)}, self),
                OpCode::OpBe => binary!(|x,y|{Value::Bool(x >= y)}, self),
                OpCode::OpLt => binary!(|x,y|{Value::Bool(x < y)}, self),
                OpCode::OpLe => binary!(|x,y|{Value::Bool(x <= y)}, self),
                OpCode::OpAnd => binary!(|x,y|{x & y}, self),
                OpCode::OpNand => binary!(|x:Value,y:Value|{!(x & y)}, self),
                OpCode::OpOr => binary!(|x,y|{x | y}, self),
                OpCode::OpNor => binary!(|x:Value,y:Value|{!(x | y)}, self),
                OpCode::OpXor => binary!(|x,y|{x ^ y}, self),
                OpCode::OpXnor => binary!(|x:Value,y:Value|{!(x ^ y)}, self),
                OpCode::OpSetGlobal => {
                    let v = self.stack.pop().unwrap();
                    let (_, value) = chunk.get_constant(self.ip+1);
                    self.globals.insert(value.get_str().to_string(), v.clone());
                    self.stack.push(v);
                    self.ip+=2;
                },
                OpCode::OpGetGlobal => {
                    let (_, value) = chunk.get_constant(self.ip+1);
                    let value = match self.globals.get(value.get_str()) {
                        Some(v) => v,
                        None => return Err(VMErr::RuntimeError(format!("cannot find global variable '{}'", value))),
                    };
                    self.stack.push(value.clone());
                    self.ip += 2;
                },
                OpCode::OpSetLocal => {
                    let v = self.stack.pop().unwrap();
                    let (_, value) = chunk.get_constant(self.ip+1);
                    self.set_local(chunk.get_name(), value.get_str().to_string(), v.clone());
                    self.stack.push(v);
                    self.ip+=2;
                },
                OpCode::OpGetLocal => {
                    let (_, value) = chunk.get_constant(self.ip+1);
                    let value = match self.get_local(chunk.get_name(), value.get_str().to_string()){
                        Some(v) => v,
                        None => return Err(VMErr::RuntimeError(format!("cannot find local variable '{}'", value))),
                    };
                    self.stack.push(value.clone());
                    self.ip += 2;
                },
                OpCode::OpJmpIfFalse => {
                    let idx = chunk.get_constant_index(self.ip+1);
                    let pred = self.stack.pop().unwrap();
                    if !pred.get_bool() {
                        self.ip = idx as usize;
                    }
                    else { 
                        self.ip += 2;
                    }
                }
                OpCode::OpJmp => {
                    let idx = chunk.get_constant_index(self.ip+1);
                    self.ip = idx as usize;
                },
                OpCode::OpCall => {
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
