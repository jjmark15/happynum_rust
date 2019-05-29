
fn square_sum(n: u32) -> u32 {
    let digit_vec: Vec<u32> = n.to_string().chars().map(|c| {
        c.to_digit(10).unwrap().pow(2)
    }).collect();

    return digit_vec.iter().sum();
}

fn is_unhappy(n:u32) -> bool {
    const UNHAPPY_MARKERS: [u32; 8] = [89, 145, 42, 37, 58, 20, 4, 16];
    return UNHAPPY_MARKERS.contains(&n)
}

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
