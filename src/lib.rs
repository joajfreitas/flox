#![feature(iter_intersperse)]

pub mod chunk;
pub mod compiler;
pub mod ir;
pub mod scanner;
pub mod source_info;
pub mod stage1;
pub mod stage2;
pub mod vm;

pub fn rep(input: &str, debug: bool) -> Result<String, String> {
    let mut chk = chunk::Chunk::new("test chunk");
    let mut comp = compiler::Compiler::new(None);
    compiler::compile(input, &mut chk, &mut comp)?;

    let mut vm = vm::VirtualMachine::new(debug);
    match vm.run(&mut chk) {
        Ok(v) => Ok(format!("{}", v)),
        Err(err) => Err(format!("{:?}", err)),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_empty() {
        assert_eq!(rep("(+ 1 1)", false).unwrap(), "2.0".to_string());
    }
}
