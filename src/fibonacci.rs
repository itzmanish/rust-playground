// generate nth fibonacci number

fn main() {
    let value = fibonacci(5);
    println!("fibonacci of 5 is {}", value)
}

fn fibonacci(x: u8) -> u8 {
    if x == 0 {
        return 1;
    }
    x * fibonacci(x - 1)
}
