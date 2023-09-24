use std::fs;
use std::io::{Error, ErrorKind};

use serde::{Deserialize, Serialize};

use clap::Parser;

use flox::rep;

#[derive(Parser, Debug)]
#[clap(about, version, author)]
struct Args {
    #[clap(short, long)]
    debug: bool,
    dir: String,
}

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
    enabled: Option<bool>,
    err: Option<String>,
}

struct TestResult {
    warnings: i32,
    errors: i32,
    tests: i32,
}

fn validate_ok(output: String, test: &Test) -> (bool, bool) {
    if output == test.output {
        println!("\t✔ {}", test.name);
    } else {
        match test.enabled {
            Some(false) => {
                println!(
                    "\t ⚠️  {}:  {} -> {} | {}",
                    test.name, test.input, output, test.output
                );
                return (true, false);
            }
            Some(true) | None => {
                println!(
                    "\t ❌ {}:  {} -> {} | {}",
                    test.name, test.input, output, test.output
                );
                return (false, true);
            }
        }
    }

    (false, false)
}

fn validate_err(err: String, test: &Test) -> (bool, bool) {
    let expected_output = if test.err.is_none() {
        test.output.clone()
    } else {
        test.err.clone().unwrap()
    };

    if test.err.as_ref().is_none() || &err != test.err.as_ref().unwrap() {
        if test.enabled == Some(false) {
            println!(
                "\t ⚠️  {}:  {} -> {} | {}",
                test.name, test.input, err, expected_output
            );
            return (true, false);
        } else {
            println!(
                "\t ❌ {}:  {} -> {} | {}",
                test.name, test.input, err, expected_output
            );
            return (false, true);
        }
    }

    (true, true)
}

fn validate(output: Result<String, String>, test: &Test) -> (bool, bool) {
    match output {
        Ok(v) => validate_ok(v, test),
        Err(err) => validate_err(err, test),
    }
}

fn run_test_file(filename: &std::path::PathBuf, debug: &bool) -> TestResult {
    let source = fs::read_to_string(filename).unwrap();
    let suites: Vec<Suite> = serde_json::from_str(&source).unwrap();

    let mut error_count = 0;
    let mut warning_count = 0;
    let mut test_count = 0;

    for suite in suites {
        println!("- {}:", suite.name);
        for test in suite.tests {
            test_count += 1;
            let output = rep(&test.input, *debug);
            let (warn_inc, error_inc) = validate(output, &test);
            if warn_inc {
                warning_count += 1;
            }
            if error_inc {
                error_count += 1;
            }
        }
    }

    TestResult {
        warnings: warning_count,
        errors: error_count,
        tests: test_count,
    }
}

fn main() -> Result<(), std::io::Error> {
    let args = Args::parse();
    let results = fs::read_dir(args.dir.clone())
        .unwrap()
        .map(|path| run_test_file(&path.unwrap().path(), &args.debug))
        .reduce(|x, y| TestResult {
            warnings: x.warnings + y.warnings,
            errors: x.errors + y.errors,
            tests: x.tests + y.tests,
        })
        .unwrap();

    println!("\n⚠️  warnings: {}/{}", results.warnings, results.tests);
    println!("❌ errors: {}/{}", results.errors, results.tests);

    if results.errors > 0 {
        Err(Error::new(ErrorKind::Other, "Tests failed!"))
    } else {
        Ok(())
    }
}
