use std::io;

fn main() {
    println!("This is a program to calculate how many prime numbers are there between 1 and N.");
    println!("Please input N:");
    let n_primes: usize = read_input();

    // Print input
    println!("Your input: {}", n_primes);

    // Calculate primes
    let mut primes: Vec<bool> = vec![true; n_primes];
    primes[0] = false;
    primes[1] = false;

    // Unravel some loops for faster calculations

    // Every even number after 2 is not prime
    let mut i: usize = 4;
    while i < n_primes {
        primes[i] = false;
        i += 2;
    }

    // Every multiple of 3 is not a prime number
    let mut i: usize = 9;
    while i < n_primes {
        primes[i] = false;
        // 6 by 6 as we already know that even numbers are not prime
        i += 6;
    }

    // Every prime is of the form 6k+1 or 6k-1, so iterate only over those
    let mut i: usize = 6;
    while i < n_primes {
        if primes[i - 1] {
            let mut j = (i - 1) * (i - 1);
            while j < n_primes {
                primes[j] = false;
                // multiply by 2 as we don't need to check even numbers
                j += (i - 1) * 2;
            }
        }

        if primes[i + 1] {
            let mut j = (i + 1) * (i + 1);
            while j < n_primes {
                primes[j] = false;
                // multiply by 2 as we don't need to check even numbers
                j += (i + 1) * 2;
            }
        }
        i += 6;
    }

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
