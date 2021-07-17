pub fn reply(message: &str) -> &str {
    let msg: String = message.chars().filter(|x| !x.is_whitespace()).collect();
    if msg.is_empty() {
        "Fine. Be that way!"
    } else if msg.chars().any(|x| x.is_alphabetic())
        && msg
            .chars()
            .filter(|x| x.is_alphabetic())
            .all(|x| x.is_uppercase())
    {
        if msg.ends_with('?') {
            "Calm down, I know what I'm doing!"
        } else {
            "Whoa, chill out!"
        }
    } else if msg.ends_with('?') {
        "Sure."
    } else {
        "Whatever."
    }
}
