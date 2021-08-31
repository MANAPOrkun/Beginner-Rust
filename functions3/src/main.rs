// In Rust, the return value of the function is 
// synonymous with the value of the final expression 
// in the block of the body of a function. 
// You can return early from a function by using 
// the return keyword and specifying a value, 
// but most functions return the last expression implicitly.


fn main() {
    let x = plus_one(5);
    let y = five();

    println!("The value of x is: {}, y is: {}", x, y);
}

fn plus_one(x: i32) -> i32 {
    x + 1
}

fn five() -> i32 {
    5
}