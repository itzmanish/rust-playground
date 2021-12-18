// generics , traits and lifetimes
use std::fmt::Display;

struct Pair<T> {
    x: T,
    y: T,
}

impl<T> Pair<T> {
    fn x(&self) -> &T {
        &self.x
    }
}

impl<T: Display + PartialOrd> Pair<T> {
    fn cmp_display(&self) {
        if self.x >= self.y {
            println!("The largest member is x = {}", self.x());
        } else {
            println!("The largest member is y = {}", self.y);
        }
    }
}

// largest returns largest number from list
fn largest(list: &[i32]) -> i32 {
    let mut greatest = list[0];
    for &item in list {
        if item > greatest {
            greatest = item;
        }
    }
    greatest
}

fn largest_generic<T: PartialOrd + Copy>(list: &[T]) -> T {
    let mut greatest = list[0];
    for &item in list {
        if item > greatest {
            greatest = item;
        }
    }
    greatest
}

// traits
pub trait Summary {
    // just signature
    // fn summarize(&self) -> String;
    // signature with definition for default
    fn summarize(&self) -> String {
        String::from("Read more...")
    }
}

pub struct Article {
    pub headline: String,
    pub author: String,
    pub content: String,
}

pub struct Twitter {}

impl Summary for Twitter {}

impl Summary for Article {
    fn summarize(&self) -> String {
        format!("{}, by {}", self.headline, self.author)
    }
}
// Without generics
// pub fn notify(item: &(impl Summary + Display)) {
//     println!("{}", item.summarize())
// }

// with generics
// pub fn notify<T: Summary + Display>(item: &T) {
//     println!("{}", item.summarize())
// }

// we can also use where clause which improve readablity
pub fn notify<T>(item: &T)
where
    T: Summary + Display,
{
    println!("{}", item.summarize())
}
// we can also return traits from a func
pub fn return_summarizable() -> impl Summary {
    Twitter {}
}

// explicit lifetimes
fn longest<'a, T>(x: &'a str, y: &'a str, announce: T) -> &'a str
where
    T: Display,
{
    println!("Announcement {}", announce);
    if x.len() > y.len() {
        x
    } else {
        y
    }
}

fn main() {
    let p = Pair { x: 5, y: 10 };

    println!("p.x = {}", p.x());

    // code to find largest number in a list of numbers(i32)
    let list = vec![10, 23, 123, 984, 33, 42, 5345, 34, 334];
    let largest = largest(&list);
    println!("Largest number in first list is {}", largest);

    // traits
    let article = Article {
        author: "Manish".to_string(),
        headline: "Hola!".to_string(),
        content: "Como estas!".to_string(),
    };
    println!("summary: {}", article.summarize());
    let twitter = Twitter {};
    println!("summary: {}", twitter.summarize());
    let list = vec![10, 23, 123, 984, 33, 42, 5345, 34, 334];
    let largest = largest_generic(&list);
    println!("Largest number in first list is {}", largest);
    let list_str = vec!["Hola", "Bonjor", "Ciao", "Hello"];
    println!(
        "Largest character in list is {}",
        largest_generic(&list_str)
    );

    let x = "Hola";
    {
        let y = String::from("Manish");
        let longest_value = longest(&x, y.as_str(), "Heya");
        println!("longest is {}", longest_value);
    }
}
