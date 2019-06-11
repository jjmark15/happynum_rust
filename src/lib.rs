#![crate_name = "happynum"]

/// Returns the sum of the square of all digits in `n`
fn square_sum(n: u32) -> u32 {
	let mut ss: u32 = 0;
	let mut val: u32 = n;

	while val > 0 {
		ss += (val % 10).pow(2);

		// take advantage of losing decimal values in an ineger value
		val /= 10;
	}
	return ss;
}

/// Returns `false` if number is contained in list of unhappy numbers
fn is_unhappy(n:u32) -> bool {
    const UNHAPPY_MARKERS: [u32; 8] = [89, 145, 42, 37, 58, 20, 4, 16];
    return UNHAPPY_MARKERS.contains(&n)
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
	let mut count = 0;
    for i in 1..(n + 1) {
        if is_first_it(i) && is_happy(i) {
            count += 1;
        }
    }
	return count;
}

/// Returns `true` if all digits in `n` are sorted
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
