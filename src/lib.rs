pub mod chunk;
pub mod vm;
pub mod scanner;
pub mod compiler;
pub mod ir;

pub fn rep(input: &str) -> String{
    let mut chk = chunk::Chunk::new("test chunk");
    compiler::compile(input, &mut chk);

    let mut vm = vm::VirtualMachine::new();
    match vm.interpret(&mut chk) {
        Ok(v) => format!("{}", v),
        Err(_) => "err".to_string(),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_empty() {
        assert_eq!(rep("(+ 1 1)"), "2.0".to_string());
    }

}
