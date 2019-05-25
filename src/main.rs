extern crate happynum;

fn is_first_it(n: u32) -> bool {
    let n_string: String = n.to_string();
    let digit_vec: Vec<u32> = n_string.chars()
        .map(|s| s.to_digit(10).unwrap()).collect();

    // return digit_vec.is_sorted();
    return true;
}

fn main() {
    let range = 100;
    let mut count = 0;

    for i in 1..(range + 1) {
        if is_first_it(i) && happynum::is_happy(i) {
            count += 1;
        }
    }

    println!("{}", count);
}