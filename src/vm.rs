use crate::chunk::closure::{Closure, ObjUpvalue};
use crate::chunk::object::{Function, Object};
use crate::chunk::value::Value;
use crate::chunk::{Chunk, OpCode};

struct CallFrame {
    closure: Box<Closure>,
    ip: usize,
    stackpointer: usize,
}

pub struct VirtualMachine {
    stack: Vec<Value>,
    frames: Vec<CallFrame>,
    fp: usize,
    debug: bool,
    ip: usize,
}

#[derive(Debug)]
pub enum VMErr {
    CompileError,
    RuntimeError(String),
}

macro_rules! nullary {
    ($fn:expr, $self:expr, $ip:expr) => {{
        $self.stack.push($fn());
        $self.set_ip($ip + 1);
    }};
}

macro_rules! unary {
    ($fn:expr, $self:expr, $ip:expr) => {{
        let arg = $self.stack.pop().unwrap();
        $self.stack.push($fn(arg));
        $self.set_ip($ip + 1);
    }};
}

macro_rules! binary {
    ($fn:expr, $self:expr, $ip:expr) => {{
        let arg2 = $self.stack.pop().unwrap();
        let arg1 = $self.stack.pop().unwrap();
        $self.stack.push($fn(arg1, arg2));
        $self.set_ip($ip + 1);
    }};
}

impl VirtualMachine {
    pub fn new(debug: bool) -> VirtualMachine {
        VirtualMachine {
            stack: Vec::new(),
            frames: Vec::new(),
            fp: 0,
            debug,
            ip: 0,
        }
    }

    pub fn get_chunk(&self) -> Chunk {
        self.frames[self.fp].closure.function.chunk.clone()
    }

    pub fn get_ip(&self) -> usize {
        self.frames[self.fp].ip
    }

    pub fn set_ip(&mut self, ip: usize) {
        self.frames[self.fp].ip = ip;
    }

    pub fn run(&mut self, chunk: &mut Chunk) -> Result<Value, VMErr> {
        let frame = CallFrame {
            closure: Box::new(Closure {
                function: Box::new(Function {
                    arity: 0,
                    chunk: chunk.clone(),
                    name: "main".to_string(),
                    upvalue_count: 0,
                }),
                upvalues: Vec::new(),
            }),
            ip: self.ip,
            stackpointer: 0,
        };

        self.frames.pop(); //cleanup callstack
        self.frames.push(frame);

        loop {
            let chunk = self.get_chunk();
            let ip = self.get_ip();
            if !chunk.is_ip_in_range(ip) {
                return Err(VMErr::RuntimeError(format!(
                    "Attemting to access unreachable bytecode. ip: {}, len: {}",
                    ip,
                    chunk.len()
                )));
            }
            if self.debug {
                let (s, _) = chunk.display_instruction(ip).unwrap();
                print!("===> {}", s);
                println!("______stack______");
                for stack in self.stack.iter() {
                    println!("[ {} ]", stack);
                }
                println!("_________________");
            }
            let opcode = chunk.get_opcode(ip).unwrap();
            match opcode {
                OpCode::OpRet => {
                    if self.frames.len() == 1 {
                        self.ip = self.get_ip() + 1;
                        match self.stack.last() {
                            Some(x) => return Ok(x.clone()),
                            None => return Ok(Value::Nil),
                        };
                    } else {
                        let ret = self.stack.pop().unwrap();
                        for _i in 0..(*self.frames.last().unwrap().closure).function.arity {
                            self.stack.pop();
                        }

                        self.stack.push(ret);
                        self.frames.pop();
                        self.fp -= 1;
                        self.set_ip(self.get_ip() + 1);
                    }
                }
                OpCode::OpConst => {
                    let (_, value) = chunk.get_constant(ip + 1);
                    self.stack.push(value.clone());
                    self.set_ip(ip + 2);
                }
                OpCode::OpConstLong => {
                    let value = chunk.get_constant_long(ip + 1).unwrap();
                    self.stack.push(value.clone());
                    self.set_ip(ip + 4);
                }
                OpCode::OpNil => nullary!(|| { Value::Nil }, self, ip),
                OpCode::OpTrue => nullary!(|| { Value::Bool(true) }, self, ip),
                OpCode::OpFalse => nullary!(|| { Value::Bool(false) }, self, ip),
                OpCode::OpAdd => binary!(|x, y| { x + y }, self, ip),
                OpCode::OpSub => binary!(|x, y| { x - y }, self, ip),
                OpCode::OpMul => binary!(|x, y| { x * y }, self, ip),
                OpCode::OpDiv => binary!(|x, y| { x / y }, self, ip),
                OpCode::OpNot => unary!(|x: Value| { !x }, self, ip),
                OpCode::OpEq => binary!(|x, y| { Value::Bool(x == y) }, self, ip),
                OpCode::OpNe => binary!(|x, y| { Value::Bool(x != y) }, self, ip),
                OpCode::OpBt => binary!(|x, y| { Value::Bool(x > y) }, self, ip),
                OpCode::OpBe => binary!(|x, y| { Value::Bool(x >= y) }, self, ip),
                OpCode::OpLt => binary!(|x, y| { Value::Bool(x < y) }, self, ip),
                OpCode::OpLe => binary!(|x, y| { Value::Bool(x <= y) }, self, ip),
                OpCode::OpAnd => binary!(|x, y| { x & y }, self, ip),
                OpCode::OpNand => binary!(|x: Value, y: Value| { !(x & y) }, self, ip),
                OpCode::OpOr => binary!(|x, y| { x | y }, self, ip),
                OpCode::OpNor => binary!(|x: Value, y: Value| { !(x | y) }, self, ip),
                OpCode::OpXor => binary!(|x, y| { x ^ y }, self, ip),
                OpCode::OpXnor => binary!(|x: Value, y: Value| { !(x ^ y) }, self, ip),
                OpCode::OpSetLocal => {
                    let value = self.stack.last().unwrap();
                    let slot = chunk.get_constant_index(ip + 1);
                    self.stack[slot as usize] = value.clone();
                    self.set_ip(ip + 2);
                }
                OpCode::OpGetLocal => {
                    let slot = chunk.get_constant_index(ip + 1);
                    let fp = self.frames.last().unwrap().stackpointer;
                    let id = slot as usize + fp;
                    if id >= self.stack.len() {
                        return Err(VMErr::RuntimeError(String::from("Out of bound access")));
                    }
                    self.stack.push(self.stack[id].clone());
                    self.set_ip(ip + 2);
                }
                OpCode::OpJmpIfFalse => {
                    let idx = chunk.get_constant_index(ip + 1);
                    let pred = self.stack.pop().unwrap();
                    if !pred
                        .get_bool()
                        .ok_or_else(|| VMErr::RuntimeError("Failed to get boolean".to_string()))?
                    {
                        self.set_ip(idx as usize);
                    } else {
                        self.set_ip(ip + 2);
                    }
                }
                OpCode::OpJmp => {
                    let idx = chunk.get_constant_index(ip + 1);
                    self.set_ip(idx as usize);
                }
                OpCode::OpCall => {
                    let mut args: Vec<Value> = Vec::new();
                    loop {
                        let v = self.stack.pop().unwrap();
                        if v.is_closure() {
                            let f = v.get_closure().ok_or_else(|| {
                                VMErr::RuntimeError("Failed to find closure".to_string())
                            })?;

                            let frame = CallFrame {
                                closure: f.clone(),
                                ip: 0,
                                stackpointer: self.stack.len(),
                            };

                            println!("~~~~~~~~~~~~~~");
                            println!("{}, ", f.function.chunk);
                            println!("~~~~~~~~~~~~~~");

                            self.frames.push(frame);
                            self.fp += 1;
                            break;
                        } else {
                            args.push(v);
                        }
                    }
                    for arg in args.iter().rev() {
                        self.stack.push(arg.clone());
                    }
                }
                OpCode::OpGetUpvalue => {
                    let slot = chunk.get_constant_index(ip + 1);
                    self.stack.push(
                        dbg!(
                            &self.stack[dbg!(
                                &self
                                    .frames
                                    .get(self.fp)
                                    .expect("expected frame")
                                    .closure
                                    .upvalues
                            )
                            .get(dbg!(slot))
                            .expect("expected slot")
                            .location]
                        )
                        .clone(),
                    );
                    self.set_ip(ip + 2);
                }
                OpCode::OpSetUpvalue => {
                    unimplemented!();
                }
                OpCode::OpClosure => {
                    let function = chunk.get_constant(ip + 1).1;

                    let mut closure = Closure {
                        function: function.get_function().unwrap(),
                        upvalues: Vec::new(),
                    };
                    let function = function.get_function().unwrap();
                    for i in 0..function.upvalue_count {
                        let is_local = chunk.get_constant_index(ip + 2 * i + 2);
                        let index = chunk.get_constant_index(ip + 2 * i + 3);

                        if is_local == 1 {
                            closure.upvalues.push(ObjUpvalue {
                                location: dbg!(self.frames[self.fp].stackpointer + index),
                            });
                        }
                    }
                    self.stack
                        .push(Value::Obj(Box::new(Object::Closure(Box::new(closure)))));
                    self.set_ip(ip + 2 + 2 * function.upvalue_count);
                }
                OpCode::OpPrint => unary!(
                    |x| {
                        println!("{:?}", x);
                        x
                    },
                    self,
                    ip
                ),
            }
        }
    }
}

impl Default for VirtualMachine {
    fn default() -> Self {
        Self::new(false)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_empty() {
        let mut vm = VirtualMachine::new(false);
        let mut chunk = Chunk::new("test");
        assert_eq!(vm.run(&mut chunk).is_err(), true);
    }

    #[test]
    fn test_basic() {
        let mut vm = VirtualMachine::new(false);
        let mut chunk = Chunk::new("test");
        chunk.write_opcode(OpCode::OpRet, 1);
        vm.run(&mut chunk).unwrap();
    }
}
