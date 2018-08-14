fn div(q: i32, n: i32) -> bool {
    n % q == 0
}

fn main() {
    for i in 1..=100 {
        if div(3, i) {
            print!("Fizz");
        }
        if div(5, i) {
            print!("Buzz");
        }
        if !(div(3, i) || div(5, i)) {
            print!("{}", i);
        }
        println!();
    }
}