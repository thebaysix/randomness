use std::env;
use rand::Rng;

fn main() {
    let args: Vec<String> = env::args().collect();
    println!("{:?}", args);

    // Step 1: Sieve of Eratosthenes to generate primes
    let max_str = &args[1];
    let n = max_str.parse::<u32>().unwrap();
    println!("Generating primes up to {} exclusive", n);

    let mut composites: Vec<u32> = vec![];
    let mut primes: Vec<u32> = vec![];
    let mut p = 2;
    while p < n {
        // Add p to the list of primes
        primes.push(p);

        // Go through each multiple of p, from p^2 up to n (p^2, p*(p+1), p*(p+2)... n)
        // and mark each as composite
        for i in p..=(n/p) {
            composites.push(i*p);
        }

        // Find the smallest number greater than p which is NOT in composites
        // That is the next prime
        let mut nextp = p+1;
        while nextp < n && composites.contains(&nextp) {
            nextp+=1;
        }

        p = nextp;
    }

    // Step 2: Select a random prime
    let num_primes = primes.len();
    println!("Selecting one of {} primes. Range is [{},{}].", num_primes, primes[0], primes[num_primes-1]);

    let mut rng = rand::thread_rng();
    let rand_index = rng.gen_range(0..num_primes);
    let selected_prime = primes[rand_index];
    println!("{}", selected_prime);
}
