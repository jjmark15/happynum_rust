use std::time::{Duration, Instant};

use structopt::StructOpt;

use crate::application::ApplicationService;

/// Finds distinct happy numbers within a range
#[derive(StructOpt, Debug)]
#[structopt(name = "Distinct happy number finder")]
pub(crate) struct CliOptions {
    /// Number range to search through
    #[structopt(short, long, default_value = "1")]
    range: f64,
}

pub(crate) fn run_cli(application_service: &ApplicationService) {
    let opts = CliOptions::from_args();

    let (count, duration) = time_operation(|| {
        application_service.count_distinct_happy_numbers_in_range(opts.range as u64)
    });

    println!("count:\t{}\ntime:\t{:?}", count, duration);
}

fn time_operation<T>(operation: impl Fn() -> T) -> (T, Duration) {
    let start = Instant::now();
    let output = operation();
    let duration: Duration = start.elapsed();
    (output, duration)
}
