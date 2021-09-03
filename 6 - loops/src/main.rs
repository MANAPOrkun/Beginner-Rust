use std::io;

fn main() {

    loop{
        println!("Please select an option (a. Loop b. While c. For q. Quit): ");

        let mut selection = String::new();

        io::stdin().read_line(&mut selection).expect("Failed to read line");

        let selection = selection.trim();

        if selection == "a" {
            just_loop();
        } else if selection == "b" {
            just_while();
        } else if selection == "c" {
            just_for();
        } else if selection == "q" {
            break;
        } else {
            println!("Selection error")
        }
    }

}

// While True
fn just_loop() {
    let mut counter = 0;

    let result = loop {
        counter += 1;

        if counter == 10 {
            break counter * 2;
        }
    };

    println!("The result is {}", result);

    //loop {
    //    println!("Hey, I am just a loop!");
    //}
}

fn just_while() {
    let a = [10, 20, 30, 40, 50];
    let mut index = 0;

    while index < 5 {
        println!("the value is: {}", a[index]);

        index += 1;
    }
}

fn just_for() {
    let a = [12, 23, 34, 45, 56];

    for element in a.iter() {
        println!("the value is: {}!", element);
    }

    for number in (1..4).rev() {
        println!("{}!", number);
    }
    println!("LIFTOFF!!!");
}