/*
 The main benefit of using methods instead of functions,
 in addition to using method syntax and not having to repeat 
 the type of self in every method’s signature, is for organization. 
 We’ve put all the things we can do with an instance of a type in one 
 impl block rather than making future users of our code search for capabilities
 of Rectangle in various places in the library we provide. 
*/

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
/*
 We’ve chosen &self here for the same reason we used &Rectangle
 in the function version: we don’t want to take ownership,
 and we just want to read the data in the struct, not write to it.
 If we wanted to change the instance that we’ve called the method on
 as part of what the method does, we’d use &mut self as the first parameter.
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