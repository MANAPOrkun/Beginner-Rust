fn main() {

    // Statements are instructions 
    // that perform some action 
    // and do not return a value.
    let x = 5;

    // Expressions evaluate to a 
    // resulting value.
    let y = {
        let x = 3;

        // Expressions do not include ending semicolons. 
        // If you add a semicolon to the end of an expression, 
        // you turn it into a statement, 
        // which will then not return a value.
        x + 1
    };

    println!("Y: {}", y);
}
