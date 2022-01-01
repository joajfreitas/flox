use std::env;
use std::fs;
use std::io;

use rustyline::error::ReadlineError;
use rustyline::Editor;

use flox::chunk::{Chunk, OpCode, Value};
use flox::compiler::compile;
use flox::vm::{VirtualMachine, VMErr};

fn repl() {
    let mut rl = Editor::<()>::new();
    rl.load_history(".flang-history");
    let mut prompt: String = "user> ".to_string();
    let mut vm = VirtualMachine::new();

    loop {
        let line = rl.readline(&prompt);

        match line {
            Ok(line) => {
                rl.add_history_entry(line.as_str());
                rl.save_history(".flang-history").unwrap();

                let mut chunk = Chunk::new("test chunk");
                compile(&line, &mut chunk);
                println!("{}", chunk);
                match vm.interpret(&mut chunk) {
                    Ok(v) => println!("{}", v),
                    Err(VMErr::RuntimeError(s)) => {
                        println!("Error: {}", s);
                        continue;
                    },
                    _ => continue,
                };
            },
            Err(ReadlineError::Interrupted) => break,
            Err(ReadlineError::Eof) => break,
            Err(err) => {
                println!("Error: {:?}", err);
                break
            }
        }
    }
}

fn run_file(filename: String) {
    let source = fs::read_to_string(filename).unwrap();

    let mut chunk = Chunk::new("test chunk");
    compile(&source, &mut chunk);
    println!("{}", chunk);
    let mut vm = VirtualMachine::new();
    vm.interpret(&mut chunk);
}

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() == 2 {
        run_file(args[1].clone());
    }
    else if args.len() == 1 {
        repl();
    }
}
