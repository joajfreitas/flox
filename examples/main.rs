use std::fs;

use rustyline::error::ReadlineError;
use rustyline::Editor;

use clap::Parser;

use flox::chunk::Chunk;
use flox::compiler::{compile, Compiler};
use flox::vm::{VMErr, VirtualMachine};

#[derive(Parser, Debug)]
#[clap(about, version, author)]
struct Args {
    #[clap(short, long)]
    debug: bool,
    file: Option<String>,
}

fn repl(debug: bool) {
    let mut rl = Editor::<()>::new();
    rl.load_history(".flang-history").unwrap();
    let prompt: String = "user> ".to_string();
    let mut vm = VirtualMachine::new(debug);
    //let mut comp = Compiler::new();
    let mut comp = Compiler::new(None, "main");
    let mut chunk = Chunk::new("test chunk");

    loop {
        let line = rl.readline(&prompt);

        match line {
            Ok(line) => {
                rl.add_history_entry(line.as_str());
                rl.save_history(".flang-history").unwrap();

                match compile(&line, &mut chunk, &mut comp) {
                    Err(err) => {
                        println!("{}", err);
                        continue;
                    }
                    _ => {}
                };
                if debug {
                    println!("{}", chunk);
                }
                match vm.run(&mut chunk) {
                    Ok(v) => println!("{}", v),
                    Err(VMErr::RuntimeError(s)) => {
                        println!("Error: {}", s);
                        continue;
                    }
                    _ => continue,
                };
            }
            Err(ReadlineError::Interrupted) => break,
            Err(ReadlineError::Eof) => break,
            Err(err) => {
                println!("Error: {:?}", err);
                break;
            }
        }
    }
}

fn run_file(filename: String, debug: bool) {
    let source = fs::read_to_string(filename).unwrap();

    let mut chunk = Chunk::new("test chunk");
    //let mut comp = Compiler::new();
    let mut comp = Compiler::new(None, "main");
    compile(&source, &mut chunk, &mut comp).unwrap();
    //compile(&source, &mut chunk, &mut comp);
    println!("{}", chunk);
    let mut vm = VirtualMachine::new(debug);
    vm.run(&mut chunk).unwrap();
}

fn main() {
    let args = Args::parse();
    if args.file.is_some() {
        run_file(args.file.unwrap(), args.debug);
    } else {
        repl(args.debug);
    }
}
