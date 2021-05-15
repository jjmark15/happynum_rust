#[cfg(feature = "parallel")]
use rayon::prelude::*;

/// Returns the sum of the square of all digits in `n`
fn square_sum(n: u64) -> u64 {
    let mut ss = 0;
    let mut val = n;

    while val > 0 {
        ss += (val % 10).pow(2);

        // take advantage of losing decimal values in an integer value
        val /= 10;
    }
    ss
}

fn is_unhappy(n: u64) -> bool {
    const UNHAPPY_MARKERS: [u64; 8] = [89, 145, 42, 37, 58, 20, 4, 16];
    UNHAPPY_MARKERS.contains(&n)
}

pub fn is_happy(n: u64) -> bool {
    let mut ss = square_sum(n);

    loop {
        if ss == 1 {
            return true;
        } else if is_unhappy(ss) {
            return false;
        } else {
            ss = square_sum(ss);
        }
    }
}

/// Counts distinct happy numbers in a range
#[cfg(feature = "parallel")]
pub fn count_distinct_happy_numbers_in_range(n: u64) -> u64 {
    (1..(n + 1))
        .into_par_iter()
        .filter(|&a| is_first_it(a) && is_happy(a))
        .count() as u64
}

/// Counts distinct happy numbers in a range
#[cfg(not(feature = "parallel"))]
pub fn count_distinct_happy_numbers_in_range(n: u64) -> u64 {
    (1..(n + 1))
        .into_iter()
        .filter(|&a| is_first_it(a) && is_happy(a))
        .count() as u64
}

/// Returns `true` if all digits in `n` are sorted
fn is_first_it(n: u64) -> bool {
    let mut prev: char = 0 as char;
    for (i, curr) in n.to_string().chars().enumerate() {
        if i > 0 && prev > curr {
            return false;
        }
        prev = curr;
    }
    true
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn counts_distinct_happy_numbers_in_range() {
        assert_eq!(count_distinct_happy_numbers_in_range(1000000), 711)
    }
}
