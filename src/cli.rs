use std::time::{Duration, Instant};

use clap::Parser;

use crate::happynum::{count_distinct_happy_numbers_in_range, count_distinct_happy_numbers_in_range_parallel};

/// Finds distinct happy numbers within a range
#[derive(Parser, Debug)]
#[structopt(name = "Distinct happy number finder")]
pub(crate) struct CliOptions {
    /// Number range to search through
    #[clap(short, long, default_value = "1")]
    range: f64,

    /// Run in parallel
    #[clap(short, long)]
    single_threaded: bool,
}

pub(crate) fn run_cli() {
    let opts = CliOptions::parse();

    let (count, duration) = time_operation(|| {
        if opts.single_threaded {
            count_distinct_happy_numbers_in_range(opts.range as usize)
        } else {
            count_distinct_happy_numbers_in_range_parallel(opts.range as usize)
        }
    });

    println!("count:\t{}\ntime:\t{:?}", count, duration);
}

fn time_operation<T>(operation: impl Fn() -> T) -> (T, Duration) {
    let start = Instant::now();
    let output = operation();
    let duration: Duration = start.elapsed();
    (output, duration)
}
