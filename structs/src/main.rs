struct Rectangle {
    width: u32,
    height: u32
}

impl Rectangle {
    fn area(&self) -> u32 {
        return self.width * self.height;
    }

    fn can_hold(&self, rect2: &Rectangle) -> bool {
        return if self.area() >= rect2.area() {true} else {false}; 
    }
}

fn main() {
    let rect1 = Rectangle {
        width: 30,
        height: 50,
    };

    let rect2 = Rectangle {
        width: 15,
        height: 20,
    };

    let rect3 = Rectangle {
        width: 60,
        height: 70,
    };

    println!("Can rect1 hold rect2? {}", rect1.can_hold(&rect2));
    println!("Can rect1 hold rect3? {}", rect1.can_hold(&rect3));
    println!("Can rect3 hold rect1? {}", rect3.can_hold(&rect1));

}