use std::io;
mod sieve;

fn main() {
    println!("This is a program to calculate how many prime numbers are there between 1 and N.");
    println!("Please input N:");
    let n_primes: usize = read_input();

    // Print input
    println!("Your input: {}", n_primes);

    // Calculate time for calculations
    let start = std::time::Instant::now();
    // segment_size should be the size of the L1 cache
    // The R5 3600 has 386KB of L1 cache, so we use that for better locality
    let segment_size = (386 * 1024) / 64;
    let primes = sieve::segmented_sieve_of_eratosthenes(n_primes, segment_size);
    // let primes = sieve::sieve_of_eratosthenes(n_primes);
    let duration = start.elapsed();

    println!("Time elapsed in sieve_of_eratosthenes() is: {:?}", duration);
    println!("Calculations done. Printing results...\n");

    let num_primes = primes.len();
    // print all primes without changing line
    // for i in 0..num_primes {
    //     print!("{} ", primes[i]);
    // }

    println!("\n");
    println!("There are {} prime numbers between 1 and {}.", num_primes, n_primes);
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
