fn main() {
    let x: i32 = 5;
    make_copy(x);
    println!("{}", x);
    
    let s: String = String::from("hello");
    takes_ownership(s);

    // s is borrowed
    println!("{}", s);
}

fn takes_ownership(a_string: String){
    println!("{}", a_string);
}

fn make_copy(a_int: i32){
    println!("{}", a_int);
}
