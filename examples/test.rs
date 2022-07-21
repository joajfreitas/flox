//use std::fs;
//use std::io::{Error, ErrorKind};
//
//use serde::{Deserialize, Serialize};
//use serde_json;
//
//use clap::Parser;
//
//use flox::rep;
//
//#[derive(Parser, Debug)]
//#[clap(about, version, author)]
//struct Args {
//    #[clap(short, long)]
//    debug: bool,
//    file: String,
//}
//
//#[derive(Serialize, Deserialize)]
//struct Suite {
//    tests: Vec<Test>,
//    name: String,
//}
//
//#[derive(Serialize, Deserialize)]
//struct Test {
//    input: String,
//    output: String,
//    name: String,
//    enabled: Option<bool>,
//    err: Option<String>,
//}
//
//fn validate_ok(output: String, test: &Test) -> (bool, bool) {
//    if output == test.output {
//        println!("\t✔ {}", test.name);
//    } else {
//        match test.enabled {
//            Some(false) => {
//                println!(
//                    "\t ⚠️  {}:  {} -> {} | {}",
//                    test.name, test.input, output, test.output
//                );
//                return (true, false);
//            }
//            Some(true) | None => {
//                println!(
//                    "\t ❌ {}:  {} -> {} | {}",
//                    test.name, test.input, output, test.output
//                );
//                return (false, true);
//            }
//        }
//    }
//
//    return (false, false);
//}
//
//fn validate_err(err: String, test: &Test) -> (bool, bool) {
//    let expected_output = if test.err.is_none() {
//        test.output.clone()
//    } else {
//        test.err.clone().unwrap()
//    };
//
//    if test.err.as_ref().is_none() || &err != test.err.as_ref().unwrap() {
//        if test.enabled == Some(false) {
//            println!(
//                "\t ⚠️  {}:  {} -> {} | {}",
//                test.name, test.input, err, expected_output
//            );
//            return (true, false);
//        } else {
//            println!(
//                "\t ❌ {}:  {} -> {} | {}",
//                test.name, test.input, err, expected_output
//            );
//            return (false, true);
//        }
//    }
//
//    (true, true)
//}
//
//fn validate(output: Result<String, String>, test: &Test) -> (bool, bool) {
//    match output {
//        Ok(v) => validate_ok(v, test),
//        Err(err) => validate_err(err, test),
//    }
//}
//
//fn main() -> Result<(), std::io::Error> {
//    let args = Args::parse();
//    println!("{:?}", args);
//
//    let source = fs::read_to_string(args.file).unwrap();
//    let suites: Vec<Suite> = serde_json::from_str(&source).unwrap();
//
//    let mut error_count = 0;
//    let mut warning_count = 0;
//    let mut test_count = 0;
//
//    for suite in suites {
//        println!("- {}:", suite.name);
//        for test in suite.tests {
//            test_count += 1;
//            let output = rep(&test.input, args.debug);
//            let (warn_inc, error_inc) = validate(output, &test);
//            if warn_inc {
//                warning_count += 1;
//            }
//            if error_inc {
//                error_count += 1;
//            }
//        }
//    }
//
//    println!("\n⚠️  warnings: {}/{}", warning_count, test_count);
//    println!("❌ errors: {}/{}", error_count, test_count);
//
//    if error_count > 0 {
//        Err(Error::new(ErrorKind::Other, "Tests failed!"))
//    } else {
//        Ok(())
//    }
//}
fn main() {
    println!("hello world\n");
}
