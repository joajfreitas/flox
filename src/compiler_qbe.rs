use std::collections::HashMap;

use crate::qbe;

use crate::compiler::Compiler;
use crate::parser::ast::Ast;

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
        compiler.builtins.insert("-".to_string(), qbe::Opcode::Sub);
        compiler.builtins.insert("*".to_string(), qbe::Opcode::Mul);
        compiler.builtins.insert("/".to_string(), qbe::Opcode::Div);

        compiler
    }

    fn get_builtin(&self, name: &str) -> &qbe::Opcode {
        self.builtins.get(name).unwrap()
    }

    fn get_result(&self) -> String {
        format!("{}", self.program)
    }

    fn compile_function_dec(&mut self, nodes: &Vec<Ast>) {
        let op = nodes[0].get_sym();
        if op == "fn" {
            let name = nodes[1].get_sym();
            let args = nodes[2].get_list();
            let body = &nodes[3];

            let mut function = qbe::Function::new(name, Some(qbe::Type::Word), vec![], true);
            let mut stmt = self.compile_statement(body);
            stmt.set_ret(Some(qbe::Value::local("__ret", qbe::Type::Long)));
            function.add_statement(stmt);

            let ret = qbe::Statement::new(
                qbe::Instruction::new(
                    &qbe::Opcode::Ret,
                    vec![qbe::Value::local("__ret", qbe::Type::Long)],
                ),
                None,
            );
            function.add_statement(ret);

            self.program.add_function(&function)
        } else {
            unimplemented!();
        }
    }

    fn compile_statement(&self, ast: &Ast) -> qbe::Statement {
        let list = ast.get_list();
        let opcode = self.get_builtin(list[0].get_sym());
        let args: Vec<qbe::Value> = list[1..].iter().map(|el| self.parse_value(el)).collect();

        let instr = qbe::Instruction::new(opcode, args);
        qbe::Statement::new(instr, None)
    }

    fn compile_stmt(&mut self, ast: &Ast) {
        match ast {
            Ast::List(list) => self.compile_function_dec(list),
            _ => unimplemented!(),
        }
    }

    fn parse_value(&self, value: &Ast) -> qbe::Value {
        match value {
            Ast::Int(i) => qbe::Value::ConstLong(*i as u64),
            Ast::Double(i) => qbe::Value::ConstDouble(*i as f64),
            _ => unimplemented!(),
        }
    }

    //fn parse_builtin(&mut self, ast: &Vec<Ast>) -> qbe::Instruction {
    //    let op = ast[0].get_sym();
    //    match &op as &str {
    //        "+" => qbe::Instruction::new(
    //            qbe::Opcode::Add,
    //            vec![self.parse_value(&ast[1]), self.parse_value(&ast[2])],
    //        ),
    //        _ => unimplemented!(),
    //    }
    //}
}

impl Compiler for CompilerQbe {
    fn compile(&mut self, ast: &Ast) {
        match ast {
            Ast::Block(name, stmts) => {
                for stmt in stmts {
                    self.compile_stmt(stmt);
                }
            }
            //Ast::List(list) => {
            //    assert!(list.len() != 0);
            //    let op = list[0].get_sym();
            //    if op == "fn" {
            //        let name = list[1].get_sym();
            //        let mut function = qbe::Function::new(name, None, vec![], true);
            //        let instruction = qbe::Instruction::new(
            //            qbe::Opcode::Add,
            //            vec![qbe::Value::ConstLong(1), qbe::Value::ConstLong(1)],
            //        );
            //        let stmt = qbe::Statement::new(instruction, None);
            //        function.add_statement(stmt);
            //        self.program.add_function(&function)
            //    }
            //    //match &list[0] {
            //    //    Ast::Sym(sym) => {
            //    //        if self.builtins.contains_key(&sym.clone()) {
            //    //            let instruction = self.parse_builtin(list);
            //    //            main.add_statement(qbe::Statement::new(instruction, None));
            //    //        }
            //    //    }
            //    //    _ => unimplemented!(),
            //    //}
            //}
            _ => {}
        };
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use rstest::*;

    #[fixture]
    fn compiler() -> CompilerQbe {
        CompilerQbe::new()
    }

    #[test]
    fn test_init() {
        CompilerQbe::new();
    }

    #[ignore]
    #[test]
    fn test_with_nil_input() {
        let mut compiler = CompilerQbe::new();
        compiler.compile(&Ast::nil());
        assert_eq!(
            compiler.get_result(),
            "export function w $main() {\n@start\n\tret 0\n\n}\n".to_string()
        );
    }

    #[ignore]
    #[test]
    fn test_with_sum() {
        let mut compiler = CompilerQbe::new();
        compiler.compile(&Ast::list(vec![Ast::sym("+"), Ast::int(1), Ast::int(2)]));
        assert_eq!(
            compiler.get_result(),
            "export function w $main() {\n@start\n\tadd 1, 2\n\tret 0\n\n}\n".to_string()
        );
    }

    #[ignore]
    #[test]
    fn test_with_function_def() {
        let mut compiler = CompilerQbe::new();
        compiler.compile(&Ast::list(vec![Ast::sym("fn"), Ast::sym("main")]));
        assert_eq!(
            compiler.get_result(),
            "export function $main() {\n@start\n\tadd 1, 1\n\n}\n".to_string()
        );
    }

    #[rstest]
    fn test_single_function(mut compiler: CompilerQbe) {
        compiler.compile(&Ast::block(
            "placeholder",
            &vec![Ast::List(vec![
                Ast::sym("fn"),
                Ast::sym("main"),
                Ast::List(vec![]),
                Ast::List(vec![Ast::sym("+"), Ast::Int(1), Ast::Int(2)]),
            ])],
        ));
        assert_eq!(
            compiler.get_result(),
            "export function w $main() {\n@start\n\t%__ret =l add 1, 2\n\tret %__ret\n\n}\n"
        );
    }
}
