
fn square_sum(n: u32) -> u32 {
    let n_string: String = n.to_string();
    let mut ss: u32 = 0;

    for c in n_string.chars() {
        match c.to_digit(10) {
            Some(i) => {
                ss += i.pow(2);
            },
            _ => (),
        }
    }

    return ss;
}

fn is_unhappy(n:u32) -> bool {
    let unhappy_markers: [u32; 8] = [89, 145, 42, 37, 58, 20, 4, 16];
    return unhappy_markers.contains(&n)
}

fn is_happy(n: u32) -> bool {
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

fn main() {
    let range = 10;

    for i in 1..range {
        println!("{}", is_happy(i))
    }
}
