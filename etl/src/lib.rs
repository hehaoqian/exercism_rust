use std::collections::BTreeMap;

pub fn transform(h: &BTreeMap<i32, Vec<char>>) -> BTreeMap<char, i32> {
    let mut r = BTreeMap::new();
    for (score, chars) in h.iter() {
        for c in chars.iter() {
            r.insert(c.to_ascii_lowercase(), *score);
        }
    }
    r
}
