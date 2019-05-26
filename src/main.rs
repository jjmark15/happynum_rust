extern crate happynum;

use std::time::{Duration, Instant};

fn is_first_it(n: u32) -> bool {
    let mut digit_vec: Vec<u32> = n.to_string().chars()
        .map(|s| s.to_digit(10).unwrap()).collect();
    digit_vec.sort();

    let sorted_n: u32 = {
        let sorted_n_string: String = digit_vec.iter()
            .map(|d| d.to_string()).collect();
        sorted_n_string.parse::<u32>().unwrap()
    };

    return sorted_n == n;
}

fn main() {
    let range = 1000000;
    let mut count = 0;

    let start = Instant::now();

    for i in 1..(range + 1) {
        if is_first_it(i) && happynum::is_happy(i) {
            count += 1;
        }
    }

    let duration: Duration = start.elapsed();

    println!("{} distinct happy numbers found in {:?}", count, duration);
}