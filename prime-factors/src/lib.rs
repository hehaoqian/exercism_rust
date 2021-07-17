pub fn factors(n: u64) -> Vec<u64> {
    if n <= 1 {
        return Vec::new();
    }
    let mut cur_factor = 2;

    let mut v = Vec::new();
    let mut s = n;
    while s > 1 {
        while s % cur_factor != 0 {
            cur_factor += 1;
        }
        v.push(cur_factor);
        s /= cur_factor;
    }
    v
}
