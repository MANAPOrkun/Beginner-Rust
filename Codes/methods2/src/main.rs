#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    fn area(&self) -> u32 {
        self.width * self.height
    }

    fn can_hold(&self, other: &Rectangle) -> bool {
        self.width > other.width && self.height > other.height
    }

    /*
     Associated functions are often used 
     for constructors that will return a new 
     instance of the struct. For example, 
     we could provide an associated function 
     that would have one dimension parameter 
     and use that as both width and height, 
     thus making it easier to create a square 
     Rectangle rather than having to specify 
     the same value twice:
    */
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
    let rect2 = Rectangle {
        width: 10,
        height: 40,
    };
    let rect3 = Rectangle {
        width: 60,
        height: 45,
    };

    let square = Rectangle::square(6);

    println!("Can rect1 hold rect2? {}", rect1.can_hold(&rect2));
    println!("Can rect1 hold rect3? {}", rect1.can_hold(&rect3));
    println!("\nMy square: {:#?}", square);
    println!("My squares area: {}", square.area());

}