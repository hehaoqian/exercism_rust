/// Check a Luhn checksum.
pub fn is_valid(code: &str) -> bool {
    let digits_and_checksum = code
        .chars()
        .rev()
        .filter(|c| !c.is_whitespace())
        .enumerate()
        .map(|(i, c)| {
            if !c.is_numeric() {
                None
            } else {
                let mut x = c.to_string().parse::<u8>().unwrap();
                if i % 2 == 1 {
                    x *= 2;
                    if x > 9 {
                        x -= 9;
                    }
                }
                Some(x)
            }
        })
        .try_fold((0, 0), |acc, x| Some((acc.0 + 1, acc.1 + x?)))
        .unwrap_or((0, 0));
    digits_and_checksum.0 >= 2 && digits_and_checksum.1 % 10 == 0
}
