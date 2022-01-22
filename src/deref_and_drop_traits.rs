// Deref and drop traits on custom Box<T>
use std::ops::Deref;

#[derive(Debug)]
struct MyBox<T: std::fmt::Display>(T);

impl<T: std::fmt::Display> MyBox<T> {
    fn new(x: T) -> MyBox<T> {
        MyBox(x)
    }
}

impl<T: std::fmt::Display> std::fmt::Display for MyBox<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl<T: std::fmt::Display> Deref for MyBox<T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl<T: std::fmt::Display> Drop for MyBox<T> {
    fn drop(&mut self) {
        println!("Destroying MyBox which has data: {}", &self.0);
    }
}

fn print(v: &str) {
    println!("{}", v);
}

fn main() {
    let x = MyBox::new(5);
    assert_eq!(5, *x);
    let string = MyBox::new(String::from("Hello"));
    print(&string);
    drop(x);
    println!("freed memory taken by x manually");
}
