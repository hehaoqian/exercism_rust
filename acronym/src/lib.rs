#[derive(PartialEq)]
enum CharType {
    Separator,
    LowerLetter,
    UpperLetter,
    Quote,
}

fn get_char_type(c: char) -> CharType {
    use CharType::*;
    match c {
        '\'' => Quote,
        _ if c.is_uppercase() => UpperLetter,
        _ if c.is_lowercase() => LowerLetter,
        _ => Separator,
    }
}

pub fn abbreviate(phrase: &str) -> String {
    use CharType::*;
    let mut s = String::new();

    if let Some(index) = phrase.find(':') {
        let prefix = &phrase[..index];
        if prefix.chars().all(|c| c.is_uppercase()) {
            return prefix.to_string();
        }
    }

    let mut prev_inserted_index = 0usize;
    let mut prev_char_type = Separator;
    for (i, c) in phrase.chars().enumerate() {
        let char_type = get_char_type(c);
        if char_type != prev_char_type
            && prev_char_type != Quote
            && (char_type == LowerLetter || char_type == UpperLetter)
            && prev_inserted_index + 1 != i
        {
            s.push(c.to_ascii_uppercase());
            prev_inserted_index = i;
        }
        prev_char_type = char_type;
    }
    s
}
