#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    fn area(&self) -> u32{
        self.width * self.height
    }

    fn can_hold(&self, other: &Rectangle) -> bool {
        self.width > other.width && self.height > other.height
    }

    fn square(size: u32) -> Rectangle {
        Rectangle { width: size, height: size }
    }
}


fn main() {
    let rect1 = Rectangle { width: 200, height: 200 };
    let rect2 = Rectangle { width: 300, height: 100 };
    let rect3 = Rectangle { width: 150, height: 100 };
    println!("The area of the rectangle is {} square pixels", area(&rect1));
    println!("Rectangle information {:?}", rect1);

    let my_square = Rectangle::square(100);
}
