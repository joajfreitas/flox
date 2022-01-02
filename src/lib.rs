pub mod chunk;
pub mod vm;
pub mod scanner;
pub mod compiler;
pub mod ir;

pub fn rep(input: &str, debug: bool) -> Result<String, String> {
    let mut chk = chunk::Chunk::new("test chunk");
    compiler::compile(input, &mut chk)?;

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
