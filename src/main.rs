extern crate happynum;

fn main() {
    let range = 100;
    let mut count = 0;

    for i in 1..range {
        if happynum::is_happy(i) {
            count += 1;
        }
    }

    println!("{}", count);
}