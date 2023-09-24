use std::fs;

use rustyline::error::ReadlineError;
use rustyline::{DefaultEditor, Result};

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

fn repl(debug: bool) -> Result<()> {
    let mut rl = DefaultEditor::new()?;
    let _ = rl.load_history(".flang-history").is_err();
    let prompt: String = "user> ".to_string();
    let mut vm = VirtualMachine::new(debug);

    let mut comp = Compiler::new(None);
    let mut chunk = Chunk::new("test chunk");

    loop {
        let line = rl.readline(&prompt);

        match line {
            Ok(line) => {
                let _ = rl.add_history_entry(line.as_str());
                rl.save_history(".flox-history").unwrap();

                if let Err(err) = compile(&line, &mut chunk, &mut comp) {
                    println!("{}", err);
                    continue;
                };
                if debug {
                    println!("{}", chunk);
                }

                match vm.run(&chunk) {
                    Ok(v) => println!("{}", v),
                    Err(VMErr::RuntimeError(s)) => {
                        println!("Error: {}", s);
                        continue;
                    }
                    _ => continue,
                };
            }
            Err(err) => {
                return Err(err);
            }
        }
    }
}

fn run_file(filename: String, debug: bool) {
    let source = fs::read_to_string(filename).unwrap();

    let mut chunk = Chunk::new("test chunk");
    let mut comp = Compiler::new(None);
    compile(&source, &mut chunk, &mut comp).unwrap();
    println!("{}", chunk);
    let mut vm = VirtualMachine::new(debug);
    println!("{:?}", vm.run(&chunk));
}

fn main() {
    let args = Args::parse();
    if args.file.is_some() {
        run_file(args.file.unwrap(), args.debug);
    } else {
        match repl(args.debug) {
            Ok(_) => {}
            Err(err) => match err {
                ReadlineError::Eof => {}
                _ => {
                    println!("Err: {}", err);
                }
            },
        }
    }
}
