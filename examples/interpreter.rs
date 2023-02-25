use std::fs;

use rustyline::error::ReadlineError;
use rustyline::{Editor, Result};

use clap::Parser;

#[derive(Parser, Debug)]
#[clap(about, version, author)]
struct Args {
    #[clap(short, long)]
    debug: bool,
    file: Option<String>,
}

use flox::interpreter::{S3, T3};
use flox::scanner::Scanner;
use flox::stage1::P1;
use flox::stage2::P2;
fn repl() -> Result<()> {
    let mut rl = Editor::<()>::new()?;
    rl.load_history(".flang-history")?;
    let prompt: String = "user> ".to_string();

    loop {
        let line = rl.readline(&prompt);

        match line {
            Ok(line) => {
                rl.add_history_entry(line.as_str());
                rl.save_history(".flang-history").unwrap();

                let mut scanner = Scanner::new(&line);
                let stage1 = P1::parse(&mut scanner).unwrap();
                let stage2 = P2::parse(&stage1).unwrap();
                let s3 = &S3::parse(&stage2).unwrap();
                println!("{}", S3::eval(s3).unwrap());
            }
            Err(ReadlineError::Interrupted) => break,
            Err(ReadlineError::Eof) => break,
            Err(err) => {
                println!("Error: {:?}", err);
                break;
            }
        }
    }

    Ok(())
}

fn run_file(filename: String, _debug: bool) {
    let source = fs::read_to_string(filename).unwrap();

    let mut scanner = Scanner::new(&source);
    let s1 = P1::parse(&mut scanner).unwrap();
    let s2 = P2::parse(&s1).unwrap();
    println!("{}", s2);
}

fn main() {
    let args = Args::parse();
    if args.file.is_some() {
        run_file(args.file.unwrap(), args.debug);
    } else {
        repl().unwrap();
    }
}
