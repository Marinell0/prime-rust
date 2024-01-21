pub fn sieve_of_eratosthenes(n_primes: usize) -> Vec<bool> {
    // Calculate primes
    let mut primes: Vec<bool> = vec![true; n_primes];
    primes[0] = false;
    primes[1] = false;

    // Unravel some loops for faster calculations

    // Calculate 2s and 3s as they do ot follow the format 6k+1 or 6k-1

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
    // Go only until sqrt(n_primes) as we already know that the rest are not prime
    let go_until: usize = ((n_primes as f64).sqrt() as usize) + 1;
    while i < go_until {
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

    return primes;
}
