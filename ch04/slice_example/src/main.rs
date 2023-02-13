fn main() {
    let s = String::from("Some awesome string");
    let word = first_word(&s);
    println!("The first word is -> {word}");

    let s2 = "Hello World"; // s2 is of type &str: it's a slice pointing to a specific point of the
    // binary. It is a immutable reference.
    let word2 = first_word(&s2);
    println!("The first word is -> {word2}");

}

fn first_word(s: &str) -> &str { // returns a string slice
    let bytes = s.as_bytes(); // Convert string to an array of bytes

    // Creates an iterator over the array of bytes
    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[..i];
        }
    }
    return &s[..]; // string slice of the complete string
}
