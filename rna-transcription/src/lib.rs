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
        let map = |x| RNA_NUCLEOTIDES[DNA_NUCLEOTIDES.iter().position(|&c| c == x).unwrap()];
        Rna::new(self.seq.chars().map(map).collect::<String>().as_str()).unwrap()
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
