#[derive(Debug)]
pub struct HighScores<'a> {
    scores: &'a [u32],
}

impl<'a> HighScores<'a> {
    pub fn new(scores: &'a [u32]) -> Self {
        Self { scores }
    }

    pub fn scores(&self) -> &[u32] {
        self.scores
    }

    pub fn latest(&self) -> Option<u32> {
        self.scores.last().copied()
    }

    pub fn personal_best(&self) -> Option<u32> {
        if self.scores.is_empty() {
            None
        } else {
            Some(self.scores.iter().fold(u32::MIN, |accum, x| accum.max(*x)))
        }
    }

    pub fn personal_top_three(&self) -> Vec<u32> {
        let mut v = Vec::from(self.scores);
        v.sort_by(|a, b| b.cmp(a));
        let mut r = vec![];
        for i in 0..3 {
            if i < v.len() {
                r.push(v[i]);
            }
        }
        r
    }
}
