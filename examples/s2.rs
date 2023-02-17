use rustyline::error::ReadlineError;
use rustyline::{Editor, Result};

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
                match P1::parse(&mut scanner) {
                    Err(err) => {
                        println!("err: {}", err);
                    }
                    Ok(ast) => match P2::parse(&ast) {
                        Err(err) => println!("err: {}", err),
                        Ok(s1) => println!("{}", s1),
                    },
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

    Ok(())
}

fn main() {
    repl().unwrap();
}
