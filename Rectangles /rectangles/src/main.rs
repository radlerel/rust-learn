#[derive(Debug)]

struct Rectangle {
    width: u32,
    height: u32, 
}

impl Rectangle {
    fn area(&self) -> u32 {
        self.width * self.height
    }

    fn can_hold(&self, another_rect: &Rectangle) -> bool {
        self.width > another_rect.width && self.height > another_rect.height
    }

    fn square(size: u32) -> Rectangle {
        Rectangle { width: size, height: size }
    }
}
fn main() {
    let rect1 = Rectangle { width: 30, height: 50 };
    let rect2 = Rectangle { width: 10, height: 40 };
    let rect3 = Rectangle { width: 60, height: 45 };
    println!("It's -> {}", rect1.area());
    println!("rect1 is -> {:?}", rect1);
    // k
    println!("Can rect1 contain rect2 in self -> {}", rect1.can_hold(&rect2));
    println!("Can rect1 conatin rect3 in self -> {}", rect1.can_hold(&rect3));
}

fn calc_area(rectangle: &Rectangle) -> u32 {
    rectangle.width * rectangle.height
}