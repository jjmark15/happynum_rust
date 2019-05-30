use clap::{Arg, App};

fn is_first_it(n: u32) -> bool {
    let mut prev: char = 0 as char;
    for (i, curr) in n.to_string().chars().enumerate() {
        if i > 0 {
            if prev > curr {
                return false;
            }
        }
        prev = curr;
    }
    return true;
}

fn run_happy_check(upper_bound: u32) {
    use std::time::{Duration, Instant};

    let start = Instant::now();

    let mut count = 0;
    for i in 1..(upper_bound + 1) {
        if is_first_it(i) && happynum::is_happy(i) {
            count += 1;
        }
    }

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

    let range_end: u32 = value_t!(matches, "range", u32).unwrap();
    run_happy_check(range_end);
}
