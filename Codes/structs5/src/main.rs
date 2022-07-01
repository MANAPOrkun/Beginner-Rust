/*
Rust does include functionality to print out 
debugging information, but we have to 
explicitly opt in to make that functionality 
available for our struct. To do that, 
we add the annotation #[derive(Debug)] just 
before the struct definition
*/
#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

fn main() {
    let rect1 = Rectangle {
        width: 30,
        height: 50,
    };


    /*
    The println! macro call will now look
    like println!("rect1 is {:?}", rect1);.
    Putting the specifier :? inside the 
    curly brackets tells println! 
    we want to use an output format 
    called Debug. The Debug trait enables us to 
    print our struct in a way that is useful 
    for developers so we can see its value 
    while we’re debugging our code.
    */ 

    println!("rect1 is {:?}", rect1);

    // Eaiser to read
    println!("rect1 is {:#?}", rect1);
}