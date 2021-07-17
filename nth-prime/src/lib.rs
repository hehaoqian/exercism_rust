pub fn nth(n: u32) -> u32 {
    let mut primes = Vec::<u32>::new();
    let mut num = 2;
    while (primes.len() as u32) <= n {
        let mut is_prime = true;
        for prime in &primes {
            if (num % prime) == 0 {
                is_prime = false;
                break;
            }
        }
        if is_prime {
            primes.push(num);
        }
        num += 1;
    }
    *primes.last().unwrap()
}
