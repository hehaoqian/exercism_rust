pub fn sum_of_multiples(limit: u32, factors: &[u32]) -> u32 {
    (1..limit).fold(0, |accum, x| {
        if factors.iter().any(|f| (f != &0) && x % f == 0) {
            accum + x
        } else {
            accum
        }
    })
}
