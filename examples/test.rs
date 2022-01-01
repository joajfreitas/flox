use std::env;
use std::fs;
use std::io;
use std::io::{Error, ErrorKind};


use serde::{Deserialize, Serialize};
use serde_json;

use colored::Colorize;

use flox::vm::VMErr;
use flox::rep;


#[derive(Serialize, Deserialize)]
struct Suite {
    tests: Vec<Test>,
    name: String,
}


#[derive(Serialize, Deserialize)]
struct Test {
    input: String,
    output: String,
    name: String, 
    enabled: Option<bool>
}

fn main() -> Result<(), std::io::Error> {
    let args: Vec<String> = env::args().collect();
    if args.len() != 2 {
        return Err(Error::new(ErrorKind::Other, "Expected a single argument"));
    }

    let source = fs::read_to_string(args[1].clone()).unwrap();
    let suites: Vec<Suite> = serde_json::from_str(&source).unwrap();

    let mut error_count = 0;
    let mut warning_count = 0;
    let mut test_count = 0;

    for suite in suites {
        println!("- {}:", suite.name);
        for test in suite.tests {
            test_count += 1;
            let output = rep(&test.input);
            if  output == test.output {
                println!("\t✔ {}", test.name);
            }
            else {
                match test.enabled {
                    Some(false) => {
                        println!("\t ⚠️  {}:  {} -> {} | {}", test.name, test.input, output, test.output);
                        warning_count += 1;
                    },
                    Some(true) | None => {
                        println!("\t ❌ {}:  {} -> {} | {}", test.name, test.input, output, test.output);
                        error_count += 1;
                    }
                }
            }
        }
    }


    println!("\n⚠️  warnings: {}/{}", warning_count, test_count);
    println!("❌ errors: {}/{}", error_count, test_count);

    if error_count > 0  {
        Err(Error::new(ErrorKind::Other, "Tests failed!"))
    }
    else {
        Ok(())
    }
}
