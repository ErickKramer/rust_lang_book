fn main() {
    let mut s = String::from("hello");
    s.push_str(", world");
    println!("{}", s);

    let mut s1 = String::from("hello");
    // let s2 = s1; // Move the heap data of s1 into s2
    let s2 = s1.clone(); // Clone the heap data of s1 to s2

    println!("{}", s1);
    println!("{}", s2);

    takes_owenership(s); // s's value moves into the function and is no longer valid.
    // println!("{}", s); // This will throw a borrow of moved value error

    let x = 4;
    makes_copy(x);
    println!("{}", x); // This is fine for an integer type
    let len = calculate_length(&mut s1);
    println!("The length of '{s1}' is {len}.");
}

fn takes_owenership(some_string: String) { // some_string comes into scope
    println!("{}", some_string);
} // some_string goes out of scope and drop is called, memory is freed

fn makes_copy(some_integer: i32) { // some_integer comes into scope
    println!("{}", some_integer);
} // some_integer goes out of scope. Nothing happens.
//

/// Method that calculates the length of a string
///
/// * `s`: reference to a String
fn calculate_length(s: &mut String) -> usize {
    s.push_str(" world");
    return s.len();
} // here s goes out of scope, but because it does not have ownership of what it referes to, it is
// not dropped.
//

