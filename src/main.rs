mod chunk;
mod vm;

use crate::chunk::{Chunk, OpCode, Value};
use crate::vm::VirtualMachine;

fn main() {
    let vm = VirtualMachine::new();
    let mut chunk = Chunk::new("test chunk");
    let constant1 = chunk.add_constant(Value::Value(2.0));
    let constant2 = chunk.add_constant(Value::Value(4.0));
    chunk.write_opcode(OpCode::OpConstant, 1);
    chunk.write_constant(constant1 as u8, 1);
    chunk.write_opcode(OpCode::OpConstantLong, 2);
    chunk.write_constant_long(constant2, 2);
    chunk.write_opcode(OpCode::OpReturn, 2);
    println!("{}", chunk);
}
