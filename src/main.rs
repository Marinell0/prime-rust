use std::io;

fn main() {
    println!("This is a program to calculate how many prime numbers are there between 1 and N.");
    println!("Please input N:");
    let n_primes: i32 = read_input();

    // Print input
    println!("Your input: {}", n_primes);
}

fn read_input() -> i32 {
    let mut input = String::new();

    match io::stdin().read_line(&mut input) {
        Ok(_n) => {
            let is_numeric = input.trim().parse::<i32>();

            match is_numeric {
                Ok(n) => {
                    return n;
                },
                Err(error) => {
                    println!("Input {} is not a number: {}", input, error);
                    println!("Using default value: 100");
                    return 100;
                }
            }
        }
        Err(error) => {
            println!("error: {error}");
            println!("Using default value: 100");
            return 100;
        },
    }
}
