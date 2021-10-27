use crate::chunk::Chunk;

pub struct VirtualMachine<'a> {
    chunk: Option<Chunk<'a>>,
}

pub enum VMErr {
    INTERPRET_OK,
    INTERPRET_COMPILE_ERROR,
    INTERPRET_RUNTIME_ERROR,
}

impl VirtualMachine<'_> {
    pub fn new() -> VirtualMachine<'static> {
        VirtualMachine {chunk: None}

    }

    pub fn interpret(&mut self, chunk: Chunk) -> Result<(), VMErr>{
        self.chunk = Some(chunk);
        Ok(())
    }


}
