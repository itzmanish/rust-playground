// implicit return, expression, statement and functions

fn main() {
    let y = 4;
    println!("current value {}", y);
    let y = implicit_return(y);
    println!("after func value {}", y);
}

fn implicit_return(x: i32) -> i32 {
    x + 1
}
