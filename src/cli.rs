use clap::{Arg, App};

fn is_sorted(v: Vec<u32>) -> bool {
    for (i, n) in v.iter().enumerate() {
        match v.get(&i -1) {
            Some(prev) => {
                if prev > n {
                    return false;
                }
            },
            _ => (),
        }
    }
    return true;
}

fn is_first_it(n: u32) -> bool {
    let digit_vec: Vec<u32> = n.to_string().chars()
        .map(|s| s.to_digit(10).unwrap()).collect();
    return is_sorted(digit_vec);
}

fn run_happy_check(upper_bound: u32) {
    use std::time::{Duration, Instant};

    let mut count = 0;

    let start = Instant::now();

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
                          .version("1.2.0") // TODO: link to git tag version
                          .author("Josh Jones <ohblonddev@gmail.com>")
                          .about("Does awesome things")
                          .arg(Arg::with_name("range")
                               .short("r")
                               .long("range")
                               .value_name("RANGE")
                               .help("Choose a range from 0 to number")
                               .takes_value(true))
                          .get_matches();

    let range_end: u32 = value_t!(matches, "range", u32).unwrap_or(1000000);
    // let range_end: u32 = range_end_str.parse::<u32>().unwrap_or(0);
    run_happy_check(range_end);
}
