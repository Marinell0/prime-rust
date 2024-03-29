pub fn sieve_of_eratosthenes(n_primes: usize) -> Vec<u64> {
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
    while i <= go_until {
        if primes[i - 1] {
            set_multiples_false(&mut primes, i - 1, n_primes);
        }

        if primes[i + 1] {
            set_multiples_false(&mut primes, i + 1, n_primes);
        }
        i += 6;
    }

    // Convert to vector of primes
    let real_primes: Vec<usize> = primes.iter().enumerate()
        .filter(|(_, &is_prime)| is_prime)
        .map(|(index, _)| index)
        .collect();

    return real_primes.iter().map(|&x| x as u64).collect();
}

pub fn segmented_sieve_of_eratosthenes(n_primes: usize, segment_size: usize) -> Vec<u64> {
    let sqrt_nprimes = (n_primes as f64).sqrt() as usize + 1;

    let look_ahead;
    if n_primes < segment_size {
        look_ahead = n_primes;
    } else {
        look_ahead = match sqrt_nprimes > segment_size {
            true => sqrt_nprimes,
            false => segment_size,
        };
    }

    let mut primes: Vec<usize> = sieve_of_eratosthenes(look_ahead).iter().map(|&x| x as usize).collect();

    let mut low = segment_size;
    let mut high = segment_size * 2;

    // let sqrt_nprimes = (n_primes as f64).sqrt() as usize + 1;
    let num_primes = primes.len();

    while low < n_primes {
        if high >= n_primes {
            high = n_primes;
        }

        let mut segment: Vec<bool> = vec![true; segment_size];

        // Look until primes.len() if len is lower than sqrt of n_primes
        // Otherwise, look until sqrt of n_primes
        // let num_primes = sqrt_nprimes.min(primes.len());
        for i in 0..num_primes {
            let mut low_limit = (low / primes[i]) * primes[i];
            if low_limit < low {
                low_limit += primes[i];
            }

            let mut j = low_limit;
            while j < high {
                segment[j - low] = false;
                j += primes[i];
            }
        }

        for i in low..high {
            if segment[i - low] {
                // Add primes to the already defined primes array
                primes.push(i);
            }
        }

        low += segment_size;
        high += segment_size;
    }
    return primes.iter().map(|&x| x as u64).collect();
}

fn set_multiples_false(primes: &mut Vec<bool>, i: usize, n_primes: usize) {
    let mut j = i * i;
    let to_add = i * 2;

    while j < n_primes {
        primes[j] = false;
        // multiply by 2 as we don't need to check even numbers
        j += to_add;
    }
}
