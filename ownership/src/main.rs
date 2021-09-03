fn main() {
    let x: i32 = 5;
    let y: i32 = x; // Copy

    let s1: String = String::from("Hello");
    let s2: String = s1; // Move

    println!("x: {}, y: {}", x, y);

    // It will give 'value borrowed here after move' error 
    // because s1 not exists anymore
    println!("s1: {}, s2: {}", s1, s2);

    // To prevent this error, .clone() method which copies
    // can be used.
}
