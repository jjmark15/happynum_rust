use std::process::exit;

use clap::{App, Arg};

use happynum::distinct_is_happy_range;

fn run_happy_check(upper_bound: u32) {
    use std::time::{Duration, Instant};

    let start = Instant::now();
    let count = distinct_is_happy_range(upper_bound);
    let duration: Duration = start.elapsed();
    println!("{} distinct happy numbers found in {:?}", count, duration);
}

fn f32_can_be_u32(n: f32) -> bool {
    n.fract() == 0.0
}

pub fn instantiate_cli() {
    let matches = App::new("Distinct Happy Number Range Counter")
        .version(crate_version!())
        .author("Josh Jones <ohblonddev@gmail.com>")
        .about("Gets the count of distinct happy numbers in a range")
        .arg(
            Arg::with_name("range")
                .short("r")
                .long("range")
                .value_name("RANGE")
                .help("Choose a range from 0 to number")
                .takes_value(true),
        )
        .get_matches();

    if matches.value_of("range").is_some() {
        let range_end: f32 = value_t!(matches, "range", f32).unwrap();

        if f32_can_be_u32(range_end) {
            run_happy_check(range_end as u32)
        } else {
            eprintln!("Error: range limit cannot be a fraction value");
            exit(1);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_f32_can_be_u32() {
        assert!(f32_can_be_u32(1f32));
        assert!(!f32_can_be_u32(1.5f32))
    }
}
