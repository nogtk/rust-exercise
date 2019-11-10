// for debug
#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

// Refactor step 3
// Use method
impl Rectangle {
    fn area(&self) -> u32 {
        self.width * self.height
    }

    fn can_hold(&self, other: &Rectangle) -> bool {
        self.width > other.width && self.height > other.height
    }
}
fn main() {
    // let width1 = 30;
    // let height1 = 50;

    // Refactor step 1
    // Use tuple
    // Tuple is useful for structuring, but access to index is implicit
    // let rect1 = (30, 50);

    // Refactor step 2
    // Use structure
    let rect1 = Rectangle { width: 30, height: 50 };
    let rect2 = Rectangle { width: 20, height: 30 };
    let rect3 = Rectangle { width: 40, height: 40 };

    println!("rect1 is {:?}", rect1);

    // 借用の形でareaに値を渡す
    // ムーブするとrect1の所有権が破棄され、利用できなくなってしまう
    println!(
        "The area of the rectangle is {} square pixels.",
        rect1.area()
    );

    println!("Can rect1 hold rect2? {}", rect1.can_hold(&rect2));
    println!("Can rect1 hold rect3? {}", rect1.can_hold(&rect3));
}

//fn area(rectangle: &Rectangle) -> u32 {
//    rectangle.width * rectangle.height
//}
