use clap::Parser;
use std::{
    fs::read_to_string,
    ops::Index,
    time::{Duration, Instant},
};

mod function;
mod funsweet;
use funsweet::{ArgType, FunSweet};

const BENCHMARK: bool = false;

#[derive(Parser)]
struct Cli {
    file_path: String,
}

fn parse_content(content: String) {
    let mut funsweet = FunSweet::new(&content);

    while funsweet.index < funsweet.content.len() {
        funsweet.next();

        if funsweet.taking_string == true && funsweet.char != "\"" {
            funsweet.accept();
        } else if funsweet.char == ";" {
            if funsweet.taking_number == true {
                funsweet.taking_number = false;
                funsweet
                    .function_args
                    .push(ArgType::Number(funsweet.word.parse().unwrap()))
            }

            if funsweet.taking_variable == true {
                funsweet.taking_variable = false;

                let k = funsweet.word.clone();
                let v = funsweet.stored.get(&k);
                match v {
                    Some(v) => {
                        funsweet.function_args.push(v.clone());
                    }
                    _ => {}
                }
            }

            if funsweet.function_active == true {
                funsweet.function_active = false;

                if funsweet.function_name == "Print" || funsweet.function_name == "Warn" {
                    function::output(&funsweet);
                } else if funsweet.function_name == "Store" {
                    function::store(&mut funsweet);
                } else if funsweet.function_name == "Drop" {
                    function::drop(&mut funsweet);
                }
            }

            funsweet.function_name = String::from("");
            funsweet.function_args.clear();
            funsweet.reset();
        } else if funsweet.char == "<" {
            let function_name = funsweet.word.trim().to_string();
            funsweet.function_name = function_name.clone();
            funsweet.reset();
            funsweet.function_active = true;
        } else if funsweet.function_active == true {
            if funsweet.char == "\"" {
                if funsweet.taking_string == true {
                    funsweet
                        .function_args
                        .push(ArgType::String(funsweet.word.clone()));
                    funsweet.taking_string = false;
                    funsweet.reset();
                } else {
                    funsweet.taking_string = true;
                    funsweet.reset();
                }
            } else if funsweet.char.chars().next().unwrap().is_numeric() {
                if funsweet.taking_number == false {
                    funsweet.taking_number = true;
                    funsweet.reset();
                }

                funsweet.accept();
            } else if funsweet.char == "," {
                if funsweet.taking_number == true {
                    funsweet.taking_number = false;
                    funsweet
                        .function_args
                        .push(ArgType::Number(funsweet.word.parse().unwrap()))
                } else if funsweet.taking_variable == true {
                    funsweet.taking_variable = false;
                    funsweet
                        .function_args
                        .push(ArgType::String(funsweet.word.clone()));
                } else if funsweet.taking_variable == true {
                    funsweet.taking_variable = false;
                    funsweet
                        .function_args
                        .push(ArgType::String(funsweet.word.clone()))
                } else if funsweet.word.is_empty() == false {
                    funsweet
                        .function_args
                        .push(ArgType::String(funsweet.word.clone()));
                }

                funsweet.reset()
            } else if funsweet.char == "!" {
                // looking for variable
                funsweet.taking_variable = true;
            } else if funsweet.taking_variable && funsweet.char.trim().is_empty() == false {
                funsweet.accept();
            }
        } else if funsweet.char.trim().is_empty() {
            // ignore
        } else {
            funsweet.accept();
        }
    }
}

fn main() {
    let cli = Cli::parse();
    let content = &read_to_string(cli.file_path).expect("to be a file");

    if BENCHMARK == false {
        parse_content(content.clone());
    } else {
        let mut benchmark_results: Vec<Duration> = Vec::new();
        let mut benchmark_index = 0;

        while benchmark_index < 5000 {
            let ran_program = Instant::now();
            parse_content(content.clone());
            benchmark_index += 1;
            benchmark_results.push(Instant::now() - ran_program);
        }

        benchmark_results.sort();

        println!(
            "---- took {:?} to run the program",
            benchmark_results.index(benchmark_results.len() / 2),
        );
    }
}
