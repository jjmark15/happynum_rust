
fn is_unhappy(n:i32) -> bool {
    let unhappy_markers: [i32; 8] = [89, 145, 42, 37, 58, 20, 4, 16];
    return unhappy_markers.contains(&n)
}

fn is_happy(n: i32) -> bool {
    if !is_unhappy(n) {
        return true
    } else {
        return false
    }
}

fn main() {
    let range = 10;

    for i in 1..range {
        println!("{}", is_happy(i))
    }
}
