use rayon::prelude::*;

/// Returns the sum of the square of all digits in `n`
fn square_sum(n: u32) -> u32 {
    let mut ss: u32 = 0;
    let mut val: u32 = n;

    while val > 0 {
        ss += (val % 10).pow(2);

        // take advantage of losing decimal values in an integer value
        val /= 10;
    }
    ss
}

/// Returns `false` if number is contained in list of unhappy numbers
fn is_unhappy(n: u32) -> bool {
    const UNHAPPY_MARKERS: [u32; 8] = [89, 145, 42, 37, 58, 20, 4, 16];
    UNHAPPY_MARKERS.contains(&n)
}

/// Checks if a number is a happy number.
///
/// # Example
///
/// ```
/// assert_eq!(true, happynum::is_happy(10));
/// assert_eq!(false, happynum::is_happy(89));
/// ```
pub fn is_happy(n: u32) -> bool {
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

/// Counts distinct happy numbers in a range.
///
/// # Example
///
/// ```
/// assert_eq!(711, happynum::distinct_is_happy_range(1000000));
/// ```
pub fn distinct_is_happy_range(n: u32) -> u32 {
    (1..(n+1)).into_par_iter().filter(|&a| {
        is_first_it(a) && is_happy(a)
    }).count() as u32
}

/// Returns `true` if all digits in `n` are sorted
fn is_first_it(n: u32) -> bool {
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
    fn test_is_first_it() {
        assert_eq!(true, is_first_it(1234));
        assert_eq!(true, is_first_it(0000));
        assert_eq!(true, is_first_it(123));
        assert_eq!(false, is_first_it(1230));
        assert_eq!(false, is_first_it(1324));
    }

    #[test]
    fn test_is_unhappy() {
        assert_eq!(true, is_unhappy(89));
    }

    #[test]
    fn test_square_sum() {
        assert_eq!(1, square_sum(1));
        assert_eq!(4, square_sum(2));
        assert_eq!(20, square_sum(204));
        assert_eq!(0, square_sum(0));
    }
}
