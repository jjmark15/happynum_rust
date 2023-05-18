use rayon::prelude::*;

/// Returns the sum of the square of all digits in `n`
fn square_sum(n: usize) -> usize {
    let mut ss = 0;
    let mut val = n;

    while val > 0 {
        ss += (val % 10).pow(2);

        // take advantage of losing decimal values in an integer value
        val /= 10;
    }
    ss
}

fn is_unhappy(n: usize) -> bool {
    const UNHAPPY_MARKERS: [usize; 8] = [89, 145, 42, 37, 58, 20, 4, 16];
    UNHAPPY_MARKERS.contains(&n)
}

pub fn is_happy(n: usize) -> bool {
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
pub fn count_distinct_happy_numbers_in_range_parallel(n: usize) -> usize {
    (1..(n + 1))
        .into_par_iter()
        .filter(|&a| is_first_it(a) && is_happy(a))
        .count()
}

/// Counts distinct happy numbers in a range
pub fn count_distinct_happy_numbers_in_range(n: usize) -> usize {
    (1..(n + 1))
        .filter(|&a| is_first_it(a) && is_happy(a))
        .count()
}

/// Returns `true` if all digits in `n` are sorted
fn is_first_it(n: usize) -> bool {
    let mut rem = n;
    let mut prev: usize = 9;

    while rem > 0 {
        let curr = rem % 10;

        if curr > prev {
            return false;
        }

        rem /= 10;
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
