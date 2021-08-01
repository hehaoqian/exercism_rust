pub fn primes_up_to(upper_bound: u64) -> Vec<u64> {
    let mut marked = Vec::new();
    marked.resize((upper_bound + 1) as usize, false);
    marked[0] = true;
    marked[1] = true;
    for num in 2..=upper_bound {
        let mut value = num * 2;
        while value <= upper_bound {
            marked[value as usize] = true;
            value += num;
        }
    }
    marked
        .into_iter()
        .enumerate()
        .filter(|(_, marked)| !*marked)
        .map(|(num, _)| num as u64)
        .collect()
}
