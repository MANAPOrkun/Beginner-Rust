fn main() {
    let mut s: String = String::from("hello world");
    let s2: &str = "hello world";

    let word: usize = first_word(&s);
    let word2: &str = first_word2(s2);

    let hello: &str = &s[..5];

    // Because hello variable sliced 's' string.
    let world: &str = &s[..];
    
    println!("s: {}, s2: {}, word: {}, word2: {}, hello: {}, world: {}",
    s, s2, word, word2, hello, world);
    
    s.clear();
}

fn first_word(s: &String) -> usize{
    let bytes: &[u8] = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' '{
            return i;
        }
    }
    s.len()
}

fn first_word2(s: &str) -> &str{
    let bytes: &[u8] = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' '{
            return &s[..i];
        }
    }
    &s[..]
}