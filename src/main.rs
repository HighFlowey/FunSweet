use clap::Parser;
use std::{
    ops::Index,
    time::{Duration, Instant},
};
mod install;

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
    install::main();

    let cli = Cli::parse();

    if BENCHMARK == false {
        parse_content(cli.file_path.clone());
    } else {
        let mut benchmark_results: Vec<Duration> = Vec::new();
        let mut benchmark_index = 0;

        while benchmark_index < BENCHMARK_AMOUNT {
            let ran_program = Instant::now();
            parse_content(cli.file_path.clone());
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
