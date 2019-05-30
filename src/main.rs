extern crate happynum;

use std::time::{Duration, Instant};

fn is_sorted(v: Vec<u32>) -> bool {
    for (i, curr) in v.iter().enumerate() {
        if i > 0 {
            if v.get(&i -1).unwrap() > curr {
                return false;
            }
        }
    }
    return true;
}

fn is_first_it(n: u32) -> bool {
    let digit_vec: Vec<u32> = n.to_string().chars()
        .map(|s| s.to_digit(10).unwrap()).collect();
    return is_sorted(digit_vec);
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