// refrences and mutable and immutable data types

fn main() {
    let mut string = String::from("Hello");
    print_str(&string);
    mutate_str(&mut string);
    print_str(&string)
}

fn print_str(s: &String) {
    println!("the string is {}", s)
}

fn mutate_str(s: &mut String) {
    s.push_str(", Manish")
}
