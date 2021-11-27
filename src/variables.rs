fn main() {
    // mutable variable
    let mut x = 5;
    println!("{}", x);
    x = 6;
    println!("{}", x);

    // COnstant variable
    const THIS_IS_CONSTANT_NUMBER: u32 = 4;
    println!("constant value: {}", THIS_IS_CONSTANT_NUMBER);

    // variable shadowing
    let y = "some";
    println!("original value: {}", y);
    let y = 0;
    println!("shadowed value: {}", y);
}
