use std::collections::HashSet;

pub fn anagrams_for<'a>(word: &str, possible_anagrams: &[&'a str]) -> HashSet<&'a str> {
    let hash_func = |x: &str| {
        let lower = x.to_lowercase();
        let mut hash = lower.chars().into_iter().collect::<Vec<_>>();
        hash.sort_unstable();
        (lower, hash)
    };
    let (lower_word, hash_word) = hash_func(word);
    possible_anagrams
        .iter()
        .filter(|x| {
            let (lower, hash) = hash_func(x);
            lower != lower_word && hash == hash_word
        })
        .copied()
        .collect()
}
