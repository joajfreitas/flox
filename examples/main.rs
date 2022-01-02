use std::env;
use std::fs;
use std::io;

use rustyline::error::ReadlineError;
use rustyline::Editor;

use clap::Parser;

use flox::chunk::{Chunk, OpCode, Value};
use flox::compiler::compile;
use flox::vm::{VirtualMachine, VMErr};

#[derive(Parser, Debug)]
#[clap(about, version, author)]
struct Args {
    #[clap(short, long)]
    debug: bool,
    file: Option<String>
}

fn repl(debug: bool) {
    let mut rl = Editor::<()>::new();
    rl.load_history(".flang-history");
    let mut prompt: String = "user> ".to_string();
    let mut vm = VirtualMachine::new(debug);

    loop {
        let line = rl.readline(&prompt);

        match line {
            Ok(line) => {
                rl.add_history_entry(line.as_str());
                rl.save_history(".flang-history").unwrap();

                let mut chunk = Chunk::new("test chunk");
                compile(&line, &mut chunk);
                println!("{}", chunk);
                match vm.run(&mut chunk) {
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

fn run_file(filename: String, debug: bool) {
    let source = fs::read_to_string(filename).unwrap();

    let mut chunk = Chunk::new("test chunk");
    compile(&source, &mut chunk);
    println!("{}", chunk);
    let mut vm = VirtualMachine::new(debug);
    vm.run(&mut chunk);
}

fn main() {
    let args = Args::parse();
    if args.file.is_some() {
        run_file(args.file.unwrap(), args.debug);
    }
    else {
        repl(args.debug);
    }
}
