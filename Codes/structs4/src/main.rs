struct Rectangle {
    width: u32,
    height: u32,
}


fn main() {
    let rect1 = (30, 50);

    println!(
        "The area of the rectangle1 is {} square pixels.",
        area(rect1)
    );

    let rect2 = Rectangle {
        width: 30,
        height: 20,
    };


    /*
    Our area function is now defined with one parameter,
    which we’ve named rectangle, whose type is an immutable 
    borrow of a struct Rectangle instance. We want to borrow 
    the struct rather than take ownership of it. 
    This way, main retains its ownership and can continue using rect1, 
    which is the reason we use the & in the function signature and where we call the function.
    */
    println!(
        "The area of the rectangle2 is {} square pixels.",
        area2(&rect2)
    );

}


// with tuples
fn area(dimensions: (u32, u32)) -> u32 {
    dimensions.0 * dimensions.1
}

// with structs
fn area2(rectangle: &Rectangle) -> u32 {
    rectangle.width * rectangle.height
}