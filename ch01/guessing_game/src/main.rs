use rand::Rng;
use std::{cmp::Ordering, io};

fn main() {
    println!("Guess the numer!");

    // Use a random number generator local to the current thread with inclusive range
    let secret_number = rand::thread_rng().gen_range(1..=100);

    println!("The secret number is: {secret_number}");
    loop {
        println!("please input your guess:");

        let mut guess = String::new();

        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line");

        // Cast the guess to i32 by shadowing the previous instance of guess
        // also handles any invalid inputs from the user
        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num, // num value that is parsed
            Err(_) => continue,
        };

        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Too small"),
            Ordering::Greater => println!("Too Big"),
            Ordering::Equal => {
                println!("You win");
                break;
            }
        }
        println!("You guessed: {guess}");
    }
}
