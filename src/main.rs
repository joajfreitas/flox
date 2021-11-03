mod chunk;
mod vm;
mod scanner;
mod compiler;

use crate::chunk::{Chunk, OpCode, Value};
use crate::compiler::compile;
use crate::vm::VirtualMachine;

fn main() {
    //let mut vm = VirtualMachine::new();
    //let mut chunk = Chunk::new("test chunk");
    //let constant1 = chunk.add_constant(Value::Value(2.0));
    //let constant2 = chunk.add_constant(Value::Value(4.0));
    //chunk.write_opcode(OpCode::OpConstant, 1);
    //chunk.write_constant(constant1 as u8, 1);
    //chunk.write_opcode(OpCode::OpConstantLong, 2);
    //chunk.write_constant_long(constant2, 2);
    //chunk.write_opcode(OpCode::OpNegate, 3);
    //chunk.write_opcode(OpCode::OpAdd, 4);
    //chunk.write_opcode(OpCode::OpReturn, 5);

    //vm.interpret(chunk);
    let mut chunk = Chunk::new("test chunk");
    compile("(+ 1 (+ 1 1))", &mut chunk);
    println!("{}", chunk);
    let mut vm = VirtualMachine::new();
    vm.interpret(chunk);
}
