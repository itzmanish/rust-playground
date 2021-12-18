// struct and methods with debug info
#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    fn square(size: u32) -> Rectangle {
        Rectangle {
            width: size,
            height: size,
        }
    }
    fn area(&self) -> u32 {
        self.width * self.height
    }
}

fn main() {
    let rect = Rectangle {
        width: 30,
        height: 50,
    };

    let sq = Rectangle::square(20);

    println!(
        "The area of the rectangle is {} and area of square is {} square pixels.",
        rect.area(),
        sq.area(),
    );
    println!("The rectangle struct is {:?}", dbg!(&rect));
    println!("The square is {:?}", dbg!(&sq));
}
