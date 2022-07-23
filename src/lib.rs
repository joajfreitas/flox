//pub mod chunk;
//pub mod ir;
pub mod compiler;
pub mod compiler_qbe;
pub mod parser;
pub mod qbe;
pub mod scanner;
//pub mod vm;

pub fn rep(input: &str, debug: bool) -> Result<String, String> {
    //let mut chk = chunk::Chunk::new("test chunk");
    //let mut comp = compiler::FloxCompiler::new(None, compiler::Ctx::TopLevel);
    //compiler::compie(input, &mut chk, &mut comp)?;

    //let mut vm = vm::VirtualMachine::new(debug);
    //match vm.run(&mut chk) {
    //    Ok(v) => Ok(format!("{}", v)),
    //    Err(err) => Err(format!("{:?}", err)),
    //}
    //
    Ok("ok".to_string())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_empty() {
        //assert_eq!(rep("(+ 1 1)", false).unwrap(), "2.0".to_string());
    }
}
