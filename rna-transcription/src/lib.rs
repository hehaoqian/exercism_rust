use std::collections::HashMap;

#[derive(Debug, PartialEq)]
pub struct Dna {
    seq: String,
}

#[derive(Debug, PartialEq)]
pub struct Rna {
    seq: String,
}

const DNA_NUCLEOTIDES: [char; 4] = ['G', 'C', 'T', 'A'];
const RNA_NUCLEOTIDES: [char; 4] = ['C', 'G', 'A', 'U'];

impl Dna {
    pub fn new(dna: &str) -> Result<Dna, usize> {
        match dna.find(|c: char| !DNA_NUCLEOTIDES.contains(&c)) {
            Some(index) => Err(index),
            None => Ok(Self {
                seq: dna.to_string(),
            }),
        }
    }

    pub fn into_rna(self) -> Rna {
        let map = DNA_NUCLEOTIDES
            .iter()
            .cloned()
            .zip(RNA_NUCLEOTIDES.iter().copied())
            .collect::<HashMap<_, _>>();
        Rna::new(
            self.seq
                .chars()
                .into_iter()
                .map(|c| *map.get(&c).unwrap())
                .collect::<String>()
                .as_str(),
        )
        .unwrap()
    }
}

impl Rna {
    pub fn new(rna: &str) -> Result<Rna, usize> {
        match rna.find(|c: char| !RNA_NUCLEOTIDES.contains(&c)) {
            Some(index) => Err(index),
            None => Ok(Self {
                seq: rna.to_string(),
            }),
        }
    }
}
