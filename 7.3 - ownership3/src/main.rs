fn main() {
    let s1: String = give_ownership();
    let s2: String = String::from("Hello");
    let s3: String = take_give_ownership(s2);

    println!("s1 = {} \n s3 = {}", s1, s3);
}

fn give_ownership() -> String {
    let some_string: String = String::from("hello");

    some_string
}

fn take_give_ownership(a_string: String) -> String {
    println!("I got and gave back {}", a_string);
    
    a_string
}