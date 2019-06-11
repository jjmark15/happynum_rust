use clap::{Arg, App};
use happynum::{distinct_is_happy_range};

fn run_happy_check(upper_bound: u32) {
    use std::time::{Duration, Instant};

    let start = Instant::now();

    let count = distinct_is_happy_range(upper_bound);

    let duration: Duration = start.elapsed();
    println!("{} distinct happy numbers found in {:?}", count, duration);
}

pub fn instantiate_cli() {
    let matches = App::new("Distinct Happy Number Range Counter")
                        .version(crate_version!())
                        .author("Josh Jones <ohblonddev@gmail.com>")
                        .about("Gets the count of distinct happy numbers in a range")
                        .arg(Arg::with_name("range")
                            .short("r")
                            .long("range")
                            .value_name("RANGE")
                            .help("Choose a range from 0 to number")
                            .takes_value(true))
                        .get_matches();

    if let Some(_) = matches.value_of("range") {
        let range_end: u32 = value_t!(matches, "range", f32).unwrap() as u32;
        run_happy_check(range_end);
    }
}
