// loop, for loop, if else, loop returning a value

fn main() {
    for number in (1..10).rev() {
        if number == 5 {
            continue;
        }
        println!("current number is {}", number);
        let x = loop {
            if number == 7 {
                break number * 2;
            }
            break number;
        };
        println!("value of x {}", x)
    }
}
