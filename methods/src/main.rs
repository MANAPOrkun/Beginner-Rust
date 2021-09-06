#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

// Implementation block

/*
 Note that we still need to use the & before self,
 just as we did in &Rectangle. Methods can take ownership of self,
 borrow self immutably as we’ve done here, or borrow self mutably,
 just as they can any other parameter.
*/
impl Rectangle {
    fn area(&self) -> u32 {
        self.width * self.height
    }
}

fn main() {
    let rect1 = Rectangle {
        width: 30,
        height: 50,
    };

    println!(
        "The area of the rectangle is {} square pixels.",
        rect1.area()
    );
}