mod sieve {
    use std::collections::HashSet;

    pub fn get_primes_to(n: u64) -> Vec<u64> {
        let mut sieve: HashSet<u64> = HashSet::new();

        for i in 2..n {
            if sieve.contains(&i) { continue; }

            let mut x = i;

            while x < n {
                x = x + i;
                sieve.insert(x);
            }
        }

        let mut primes = Vec::new();

        for i in 2..n {
            if !sieve.contains(&i) { primes.push(i); }
        }

        primes
    }
}

fn main() {
    let n_maybe = u64::from_str_radix(&std::env::args().nth(1).unwrap(), 10);
    if let Ok(n) = n_maybe {
        println!("{:?}", sieve::get_primes_to(n));
    } else {
        println!("Could not convert argument to u64!");
    }
}