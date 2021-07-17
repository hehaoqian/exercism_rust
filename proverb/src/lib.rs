pub fn build_proverb(list: &[&str]) -> String {
    let mut r = String::new();
    if list.is_empty() {
        return String::new();
    }
    for i in 0..(list.len() - 1) {
        let word = format!("For want of a {} the {} was lost.\n", list[i], list[i + 1]);
        r.push_str(&word);
    }
    r.push_str(&format!(
        "And all for the want of a {}.",
        list.first().unwrap()
    ));
    r
}
