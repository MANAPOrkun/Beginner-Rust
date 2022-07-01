/*
Unit-Like Structs Without Any Fields

These are called unit-like structs because 
they behave similarly to (), the unit type. 
Unit-like structs can be useful in situations 
in which you need to implement a trait on some 
type but don’t have any data that you want to 
store in the type itself. 

It’s possible for structs to store references 
to data owned by something else, but to do so 
requires the use of lifetimes. Lifetimes ensure 
that the data referenced by a struct is valid 
for as long as the struct is. 
*/

struct User {
    username: &str,
    email: &str,
    sign_in_count: u64,
    active: bool,
}

fn main() {
    let user1 = User {
        email: "someone@example.com",
        username: "someusername123",
        active: true,
        sign_in_count: 1,
    };
}
