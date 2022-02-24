#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    fn area(&self) -> u32 {
        self.width * self.height
    }

    fn width(&self) -> bool {
        self.width > 0
    }

    fn can_hold(&self, rect2: &Rectangle) -> bool {
        rect2.width < self.width && rect2.height < self.height
    }

    fn square(size: u32) -> Rectangle {
        Rectangle {
            width: size,
            height: size,
        }
    }
}

fn main() {
    let rect1 = Rectangle {
        width: 30,
        height: 50,
    };

    println!("rect1 is {:?}", rect1);

    println!("The area of rectangle is {} square pixels", rect1.area());

    if rect1.width() {
        println!("The rectangle has a non zero width: {}", rect1.width);
    }

    let square = Rectangle::square(10);
}
