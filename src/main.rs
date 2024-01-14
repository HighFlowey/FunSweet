use clap::Parser;
use std::{
    fs::read_to_string,
    ops::Index,
    time::{Duration, Instant},
};

mod config;
use config::{BENCHMARK, BENCHMARK_AMOUNT};

mod function;
mod funsweet;
use funsweet::parse_content;

#[derive(Parser)]
struct Cli {
    file_path: String,
}

fn main() {
    let cli = Cli::parse();
    let content = &read_to_string(cli.file_path).expect("to be a file");

    if BENCHMARK == false {
        parse_content(content.clone());
    } else {
        let mut benchmark_results: Vec<Duration> = Vec::new();
        let mut benchmark_index = 0;

        while benchmark_index < BENCHMARK_AMOUNT {
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
