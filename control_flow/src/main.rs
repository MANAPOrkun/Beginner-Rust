use std::io;

fn main() {
    println!("Please input your number: ");

    let mut number = String::new();

    io::stdin().read_line(&mut number).expect("Failed to read line");

    let number: u32 = match number.trim().parse() {
        Ok(num) => num,
        Err(_) => {
            println!("Enter a number!");
            return;
        }

    };

    let result = is_divisible(number);

    if result {
        println!("WOW");
    } else{
        println!("Meh.");
    }
}

fn is_divisible(x: u32) -> bool{

    if x % 3 == 0 && x % 2 == 0{
        println!("number is divisible by 3 and 2");
        return true;
    } else if x % 3 == 0 {
        println!("number is divisible by 3");
        return false;
    } else if x % 2 == 0 {
        println!("number is divisible by 2");
        return false;
    } else {
        println!("nope");
        return false;
    }
}