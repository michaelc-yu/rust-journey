

struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    fn area(&self) -> u32 {
        self.width * self.height
    }
    fn can_hold(&self, other: &Rectangle) -> bool {
        other.width <= self.width && other.height <= self.height
    }
}

fn main() {
    let rect1 = Rectangle { width: 10, height: 20 };
    let rect2 = Rectangle { width: 5, height: 20 };
    let rect3 = Rectangle { width: 12, height: 5 };
    println!("rect1 area: {}", rect1.area());
    println!("rect2 area: {}", rect2.area());
    println!("rect3 area: {}", rect3.area());
    println!("rect1 can hold rect2? {}", rect1.can_hold(&rect2));
    println!("rect1 can hold rect3? {}", rect1.can_hold(&rect3));
}
