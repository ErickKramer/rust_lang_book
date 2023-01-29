use std::io;

fn main() {
    loop {
        println!("Enter number of fibonacci numbers to print");
        let mut choice = String::new();
        io::stdin()
            .read_line(&mut choice)
            .expect("Failed to catch input given");

        let choice: u32 = match choice.trim().parse::<u32>() {
            Ok(num) => num,
            Err(_) => {
                println!("Input must be a number");
                continue;
            }
        };

        println!("============================================================");
        for i in 0..choice {
            let value = fibonacci(i);
            println!("Fibonacci of {i} is {value}");
        }
        println!("============================================================");
    }
}

/// Computes the fibonacci number corresponding to a given value
fn fibonacci(n: u32) -> u32 {
    if n == 0 {
        return 0;
    } else if n == 1 {
        return 1;
    } else {
        let value: u32 = fibonacci(n - 1) + fibonacci(n - 2);
        return value;
    }
}

#[cfg(test)]
mod tests {
    use super::fibonacci;

    #[test]
    fn test_fibonacci() {
        assert_eq!(fibonacci(0), 0);
        assert_eq!(fibonacci(1), 1);
        assert_eq!(fibonacci(2), 1);
        assert_eq!(fibonacci(3), 2);
        assert_eq!(fibonacci(12), 144);
        assert_eq!(fibonacci(17), 1597);
        assert_eq!(fibonacci(19), 4181);
    }
}
