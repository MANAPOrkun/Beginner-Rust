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

    // Return values must be same
    let im_just_testing = if result { "WOW!" } else { "meh." };
    
    println!("{}", im_just_testing);
}

fn is_divisible(x: u32) -> bool{

    if x % 3 == 0 && x % 2 == 0{
        print!("{}", "number is divisible by 3 and 2 ".replace('\n', " "));
        return true;
    } else if x % 3 == 0 {
        print!("{}", "number is divisible by 3 ".replace('\n', " "));
        return false;
    } else if x % 2 == 0 {
        print!("{}", "number is divisible by 2 ".replace('\n', " "));
        return false;
    } else {
        print!("{}", "nope ".replace('\n', " "));
        return false;
    }
}