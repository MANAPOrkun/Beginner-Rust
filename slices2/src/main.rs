fn main() {
    let a = [1, 2, 3, 4, 5];
    let slice = &a[0..2];

    for i in slice.iter(){
        println!("{}", i);
    }
}
