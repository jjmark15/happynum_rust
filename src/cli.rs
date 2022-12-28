use std::time::{Duration, Instant};

use clap::Parser;

use crate::happynum::{count_distinct_happy_numbers_in_range, count_distinct_happy_numbers_in_range_parallel};

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
pub(crate) struct CliOptions {
    /// Number range to search through
    #[arg(short, long, default_value_t = 1_f64)]
    range: f64,

    /// Run single-threaded
    #[arg(short, long)]
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

    println!("count:\t{count}\ntime:\t{duration:?}");
}

fn time_operation<T>(operation: impl Fn() -> T) -> (T, Duration) {
    let start = Instant::now();
    let output = operation();
    let duration: Duration = start.elapsed();
    (output, duration)
}
