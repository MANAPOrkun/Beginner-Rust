fn main() {
    let s1: String = String::from("hello");
    let len = calculate_length(&s1); // Sending the reference of s1 (Borrowing)
    println!("The length of '{}' is {}.", s1, len);
}

fn calculate_length(s: &String) ->  usize{
    let length: usize = s.len();
    length
}
