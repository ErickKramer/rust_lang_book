fn main() {
    another_function();
}

fn another_function() {
    for number in (1..=4).rev() {
        println!("{number}");
    }
}
