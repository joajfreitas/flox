use rustyline::error::ReadlineError;
use rustyline::{Editor, Result};

use flox::scanner::Scanner;
use flox::stage1::P1;
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
                        println!("{}", err);
                    }
                    Ok(ast) => {
                        println!("{}", ast);
                    }
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
