fn main() {
    let mut s1: String = String::from("Hello");

    let r1: &mut String = &mut s1;

    // cannot borrow `s` as mutable more than once at a time.
    // Rust gives this error to prevent data races at compile time.
    // A data race* occurs when there are 2 pointers to the same data
    // and when there is no mechanism to synchronize 
    // data access between 2 pointers.
    let r2: &mut String = &mut s1;

    // To fix this error, immutable variable can be used.

    println!("{}, {}", r1, r2);

    let mut s2: String = String::from("World");

    let r3: &String = &s2;

    // cannot borrow `s2` as mutable because 
    // it is also borrowed as immutable
    // It is not possible to have a mutable reference 
    // if there is an immutable reference already exists.
    let r4: &mut String = &mut s2;

    println!("{}, {}", r3, r4);

    // There is not problem if there are more than 1 immutable references.
    // However, opposite is not possible.

    // * two or more threads in a single process access
    //   the same memory location concurrently, and
    //   at least one of the accesses is for writing, and
    //   the threads are not using any exclusive locks to control
    //   their accesses to that memory.

}
