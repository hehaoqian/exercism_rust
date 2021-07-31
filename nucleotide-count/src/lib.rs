use std::collections::HashMap;

const NUCLEOTIDES: [char; 4] = ['A', 'C', 'G', 'T'];

pub fn count(nucleotide: char, dna: &str) -> Result<usize, char> {
    if !NUCLEOTIDES.contains(&nucleotide) {
        return Err('X');
    }
    let mut count = 0usize;
    for c in dna.chars() {
        if NUCLEOTIDES.contains(&c) {
            if c == nucleotide {
                count += 1;
            }
        } else {
            return Err('X');
        }
    }
    Ok(count)
}

pub fn nucleotide_counts(dna: &str) -> Result<HashMap<char, usize>, char> {
    let mut map = HashMap::new();
    for c in ['A', 'C', 'G', 'T'] {
        map.insert(c, count(c, dna)?);
    }
    Ok(map)
}
