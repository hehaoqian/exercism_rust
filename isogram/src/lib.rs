pub fn check(candidate: &str) -> bool {
    let mut chars = candidate
        .to_lowercase()
        .chars()
        .filter(|x| x.is_alphabetic())
        .collect::<Vec<_>>();
    chars.sort_unstable();
    for (i, c) in chars.iter().enumerate() {
        if i > 0 && *c == chars[i - 1] {
            return false;
        }
    }
    true
}
