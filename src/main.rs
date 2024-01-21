use std::io;
mod sieve;

fn main() {
    println!("This is a program to calculate how many prime numbers are there between 1 and N.");
    println!("Please input N:");
    let n_primes: usize = read_input();

    // Print input
    println!("Your input: {}", n_primes);

    // Calculate primes
    let primes = sieve::sieve_of_eratosthenes(n_primes);

    println!("Calculations done. Printing results...\n");

    // print all primes without changing line
    for i in 2..n_primes {
        if primes[i] {
            print!("{} ", i);
        }
    }

    println!("\n");
    println!("There are {} prime numbers between 1 and {}.", primes.iter().filter(|&&x| x).count(), n_primes);
    println!("\n");
}

fn read_input() -> usize {
    let mut input = String::new();

    match io::stdin().read_line(&mut input) {
        Ok(_n) => {
            let is_numeric = input.trim().parse::<usize>();

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
