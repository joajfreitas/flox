use std::collections::HashMap;

use crate::qbe;

use crate::compiler::Compiler;
use crate::parser::parser::Ast;

struct CompilerQbe {
    program: qbe::Program,
    builtins: HashMap<String, qbe::Opcode>,
}

impl CompilerQbe {
    fn new() -> CompilerQbe {
        let mut compiler = CompilerQbe {
            program: qbe::Program::new(),
            builtins: HashMap::new(),
        };

        compiler.builtins.insert("+".to_string(), qbe::Opcode::Add);

        compiler
    }

    fn get_result(&self) -> String {
        format!("{}", self.program)
    }

    fn parse_value(&self, value: &Ast) -> qbe::Value {
        match value {
            Ast::Int(i) => qbe::Value::ConstLong(*i as u64),
            _ => unimplemented!(),
        }
    }
    fn parse_builtin(&mut self, ast: &Vec<Ast>) -> qbe::Instruction {
        let op = ast[0].get_sym();
        match &op as &str {
            "+" => qbe::Instruction::new(
                qbe::Opcode::Add,
                vec![self.parse_value(&ast[1]), self.parse_value(&ast[2])],
            ),
            _ => unimplemented!(),
        }
    }
}

impl Compiler for CompilerQbe {
    fn compile(&mut self, ast: &Ast) {
        let mut main = qbe::Function::new("main", Some(qbe::Type::Word), vec![], true);

        match ast {
            Ast::Nil => {}
            Ast::List(list) => {
                assert!(list.len() != 0);
                match &list[0] {
                    Ast::Sym(sym) => {
                        if self.builtins.contains_key(&sym.clone()) {
                            let instruction = self.parse_builtin(list);
                            main.add_statement(qbe::Statement::new(instruction, None));
                        }
                    }
                    _ => unimplemented!(),
                }
            }
            _ => unimplemented!(),
        };

        main.add_statement(qbe::Statement::new(
            qbe::Instruction::new(qbe::Opcode::Ret, vec![qbe::Value::ConstWord(0)]),
            None,
        ));
        self.program.add_function(&main);
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_init() {
        CompilerQbe::new();
    }

    #[test]
    fn test_with_nil_input() {
        let mut compiler = CompilerQbe::new();
        compiler.compile(&Ast::nil());
        assert_eq!(
            compiler.get_result(),
            "export function w $main() {\n@start\n\tret 0\n\n}\n".to_string()
        );
    }

    #[test]
    fn test_with_sum() {
        let mut compiler = CompilerQbe::new();
        compiler.compile(&Ast::list(vec![Ast::sym("+"), Ast::int(1), Ast::int(2)]));
        assert_eq!(
            compiler.get_result(),
            "export function w $main() {\n@start\n\tadd 1, 2\n\tret 0\n\n}\n".to_string()
        );
    }
}
