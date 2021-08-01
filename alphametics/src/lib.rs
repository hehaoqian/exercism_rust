use std::collections::{HashMap, HashSet};

struct Puzzle {
    str_numbers: Vec<String>,
    letters: HashSet<char>,
}

impl Puzzle {
    fn new(input: &str) -> Self {
        let mut numbers = vec![];
        let mut letters = HashSet::<char>::new();
        let tokens = input.split(&[' ', '=', '+'][..]);
        for token in tokens {
            numbers.push(token.to_string());
            letters.extend(token.chars());
        }
        Self {
            str_numbers: numbers,
            letters,
        }
    }

    fn permuation_to_map(&self, permu: &[u8]) -> HashMap<char, u8> {
        assert!(permu.len() == self.letters.len());
        self.letters
            .iter()
            .cloned()
            .zip(permu.iter().cloned())
            .collect()
    }

    fn is_valid_solution(&self, map: &HashMap<char, u8>) -> bool {
        let mut numbers = vec![];

        for s in self.str_numbers.iter() {
            let mut num: u64 = 0;
            for (i, c) in s.chars().enumerate() {
                let digit = map.get(&c).cloned().unwrap();
                if i == 0 && digit == 0 {
                    return false;
                }
                num = num * 10 + digit as u64;
            }
            numbers.push(num);
        }
        let expected_sum = *numbers.last().unwrap();
        let actual_sum: u64 = numbers.iter().rev().skip(1).sum();
        actual_sum == expected_sum
    }

    fn permute(
        &self,
        used_digits: &mut [bool; 10],
        num_required: usize,
        permu: &mut [u8],
    ) -> Option<HashMap<char, u8>> {
        if num_required == 0 {
            let map = self.permuation_to_map(permu);
            if self.is_valid_solution(&map) {
                return Some(map);
            } else {
                return None;
            }
        }

        for i in 0..10 {
            if used_digits[i] {
                continue;
            }
            used_digits[i] = true;
            permu[permu.len() - num_required] = i as u8;
            let result = self.permute(used_digits, num_required - 1, permu);
            if let Some(x) = result {
                return Some(x);
            }
            used_digits[i] = false;
        }
        None
    }

    fn solve(&self) -> Option<HashMap<char, u8>> {
        self.permute(
            &mut [false; 10],
            self.letters.len(),
            &mut vec![0u8; self.letters.len()],
        )
    }
}

pub fn solve(input: &str) -> Option<HashMap<char, u8>> {
    Puzzle::new(input).solve()
}
