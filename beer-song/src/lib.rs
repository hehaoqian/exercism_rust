pub fn verse(n: u32) -> String {
    if n >= 3 {
        format!(
            "{} bottles of beer on the wall, {} bottles of beer.\n\
             Take one down and pass it around, {} bottles of beer on the wall.\n",
            n,
            n,
            n - 1
        )
    } else if n == 2 {
        format!(
            "{} bottles of beer on the wall, {} bottles of beer.\n\
             Take one down and pass it around, {} bottle of beer on the wall.\n",
            n,
            n,
            n - 1
        )
    } else if n == 1 {
        format!(
            "{} bottle of beer on the wall, {} bottle of beer.\n\
            Take it down and pass it around, no more bottles of beer on the wall.\n",
            n, n
        )
    } else {
        "No more bottles of beer on the wall, no more bottles of beer.\n\
        Go to the store and buy some more, 99 bottles of beer on the wall.\n"
            .to_string()
    }
}

pub fn sing(start: u32, end: u32) -> String {
    let mut s = String::new();
    for i in (end..(start + 1)).rev() {
        s.push_str(&verse(i));
        if i != end {
            s.push('\n');
        }
    }
    s
}
