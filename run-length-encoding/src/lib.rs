use std::cmp::max;

pub fn encode(source: &str) -> String {
    let mut s = String::new();
    let mut prev_char: Option<char> = None;
    let mut count = 0;
    for c in source.chars() {
        if prev_char == Some(c) {
            count += 1;
        } else {
            if count > 1 {
                s.push_str(&count.to_string());
            }
            if count > 0 {
                s.push(prev_char.unwrap());
            }
            count = 1;
            prev_char = Some(c);
        }
    }
    if count > 1 {
        s.push_str(&count.to_string());
    }
    if count > 0 {
        s.push(prev_char.unwrap());
    }
    s
}

pub fn decode(source: &str) -> String {
    let mut s = String::new();
    let mut count = 0u32;
    for c in source.chars() {
        if c.is_numeric() {
            count *= 10;
            count += c.to_digit(10).unwrap();
        } else {
            for _ in 0..max(count, 1) {
                s.push(c);
            }
            count = 0;
        }
    }
    s
}
