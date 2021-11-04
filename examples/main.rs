use rustyline::error::ReadlineError;
use rustyline::Editor;

use flox::chunk::{Chunk, OpCode, Value};
use flox::compiler::compile;
use flox::vm::VirtualMachine;

fn main() {
    let mut rl = Editor::<()>::new();
    rl.load_history(".flang-history");
    let prompt: String = "user> ".to_string();

    let mut vm = VirtualMachine::new();

    loop {
        let readline = rl.readline(&prompt);
        match readline {
            Ok(line) => {
                let mut chunk = Chunk::new("test chunk");
                compile(&line, &mut chunk);
                vm.interpret(chunk);
                rl.add_history_entry(line.as_str());
                rl.save_history(".flang-history").unwrap();
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
