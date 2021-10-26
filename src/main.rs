mod chunk;

use crate::chunk::{OpCode, Chunk};

fn main() {
    let mut chunk = Chunk::new("test chunk");
    chunk.write(OpCode::OpReturn);
    println!("{}", chunk);
}
