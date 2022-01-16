// Iterators

struct Counter {
    count: u32,
}

impl Counter {
    fn new() -> Counter {
        Counter { count: 0 }
    }
}

impl Iterator for Counter {
    type Item = u32;

    fn next(&mut self) -> Option<Self::Item> {
        if self.count < 5 {
            self.count += 1;
            Some(self.count)
        } else {
            None
        }
    }
}

fn main() {
    let x = 10;
    let closure = |z: u32| -> bool {
        println!("heya I am closure");
        z == x
    };
    let y = 10;
    println!("x==y: {}", closure(y));

    let mut counter = Counter::new();
    println!("{:?}", counter.next());
    println!("{:?}", counter.next());
    println!("{:?}", counter.next());
    println!("{:?}", counter.next());
    println!("{:?}", counter.next());
    println!("{:?}", counter.next());
    let mut ncounter = Counter::new().zip(Counter::new().skip(1));
    println!("{:?}", ncounter.next())
}
