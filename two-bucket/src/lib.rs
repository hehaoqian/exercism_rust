use std::collections::{HashMap, VecDeque};

#[derive(PartialEq, Eq, Debug, Clone, Copy)]
pub enum Bucket {
    One,
    Two,
}

/// A struct to hold your results in.
#[derive(PartialEq, Eq, Debug)]
pub struct BucketStats {
    /// The total number of "moves" it should take to reach the desired number of liters, including
    /// the first fill.
    pub moves: u8,
    /// Which bucket should end up with the desired number of liters? (Either "one" or "two")
    pub goal_bucket: Bucket,
    /// How many liters are left in the other bucket?
    pub other_bucket: u8,
}

#[derive(PartialEq, Eq, Debug, Hash, Clone, Copy)]
struct BucketSnapshot {
    size_1: u8,
    size_2: u8,
    capacity_1: u8,
    capacity_2: u8,
}

enum BucketSnapshotIterPhase {
    Empty1,
    Empty2,
    Full1,
    Full2,
    Pour1,
    Pour2,
}

impl BucketSnapshot {
    fn can_empty(&self, bucket: Bucket) -> bool {
        match bucket {
            Bucket::One => self.size_1 > 0,
            Bucket::Two => self.size_2 > 0,
        }
    }

    fn empty(mut self, bucket: Bucket) -> Self {
        match bucket {
            Bucket::One => self.size_1 = 0,
            Bucket::Two => self.size_2 = 0,
        }
        self
    }
    fn can_full(&self, bucket: Bucket) -> bool {
        match bucket {
            Bucket::One => self.size_1 < self.capacity_1,
            Bucket::Two => self.size_2 < self.capacity_2,
        }
    }

    fn full(mut self, bucket: Bucket) -> Self {
        match bucket {
            Bucket::One => self.size_1 = self.capacity_1,
            Bucket::Two => self.size_2 = self.capacity_2,
        }
        self
    }

    fn can_pour_from(&self, bucket: Bucket) -> bool {
        match bucket {
            Bucket::One => self.size_1 > 0 && self.size_2 < self.capacity_2,
            Bucket::Two => self.size_2 > 0 && self.size_1 < self.capacity_1,
        }
    }

    fn pour_from(mut self, bucket: Bucket) -> Self {
        match bucket {
            Bucket::One => {
                let size = self.size_1.min(self.capacity_2 - self.size_2);
                self.size_1 -= size;
                self.size_2 += size;
            }
            Bucket::Two => {
                let size = self.size_2.min(self.capacity_1 - self.size_1);
                self.size_2 -= size;
                self.size_1 += size;
            }
        }
        self
    }

    fn iter(&self) -> BucketSnapshotIter {
        BucketSnapshotIter {
            init: *self,
            cur: *self,
            phase: BucketSnapshotIterPhase::Empty1,
        }
    }

    fn reach_goal(&self, goal: u8) -> Option<Bucket> {
        if self.size_1 == goal {
            Some(Bucket::One)
        } else if self.size_2 == goal {
            Some(Bucket::Two)
        } else {
            None
        }
    }
}

struct BucketSnapshotIter {
    init: BucketSnapshot,
    phase: BucketSnapshotIterPhase,
    cur: BucketSnapshot,
}

impl Iterator for BucketSnapshotIter {
    type Item = BucketSnapshot;

    fn next(&mut self) -> Option<Self::Item> {
        loop {
            use Bucket::*;
            use BucketSnapshotIterPhase::*;
            match self.phase {
                Empty1 => {
                    if !self.cur.can_empty(One) {
                        self.phase = Empty2;
                        self.cur = self.init;
                    } else {
                        self.cur = self.cur.empty(One);
                        return Some(self.cur);
                    }
                }
                Empty2 => {
                    if !self.cur.can_empty(Two) {
                        self.phase = Full1;
                        self.cur = self.init;
                    } else {
                        self.cur = self.cur.empty(Two);
                        return Some(self.cur);
                    }
                }
                Full1 => {
                    if !self.cur.can_full(One) {
                        self.phase = Full2;
                        self.cur = self.init;
                    } else {
                        self.cur = self.cur.full(One);
                        return Some(self.cur);
                    }
                }
                Full2 => {
                    if !self.cur.can_full(Two) {
                        self.phase = Pour1;
                        self.cur = self.init;
                    } else {
                        self.cur = self.cur.full(Two);
                        return Some(self.cur);
                    }
                }
                Pour1 => {
                    if !self.cur.can_pour_from(One) {
                        self.phase = Pour2;
                        self.cur = self.init;
                    } else {
                        self.cur = self.cur.pour_from(One);
                        return Some(self.cur);
                    }
                }
                Pour2 => {
                    if !self.cur.can_pour_from(Two) {
                        return None;
                    } else {
                        self.cur = self.cur.pour_from(Two);
                        return Some(self.cur);
                    }
                }
            }
        }
    }
}
/// Solve the bucket problem
pub fn solve(
    capacity_1: u8,
    capacity_2: u8,
    goal: u8,
    start_bucket: &Bucket,
) -> Option<BucketStats> {
    let mut best_moves = HashMap::new();
    let mut bfs = VecDeque::new();

    let init = match start_bucket {
        Bucket::One => BucketSnapshot {
            size_1: capacity_1,
            size_2: 0,
            capacity_1,
            capacity_2,
        },
        Bucket::Two => BucketSnapshot {
            size_1: 0,
            size_2: capacity_2,
            capacity_1,
            capacity_2,
        },
    };
    let forbidden = match start_bucket {
        Bucket::One => BucketSnapshot {
            size_2: capacity_2,
            size_1: 0,
            capacity_1,
            capacity_2,
        },
        Bucket::Two => BucketSnapshot {
            size_2: 0,
            size_1: capacity_1,
            capacity_1,
            capacity_2,
        },
    };
    bfs.push_back(init);
    best_moves.insert(init, 1);

    while !bfs.is_empty() {
        let front = bfs.pop_front().unwrap();
        let moves = *best_moves.get(&front).unwrap();
        if let Some(bucket) = front.reach_goal(goal) {
            return Some(BucketStats {
                moves,
                goal_bucket: bucket,
                other_bucket: match bucket {
                    Bucket::One => front.size_2,
                    Bucket::Two => front.size_1,
                },
            });
        }
        for stat in front.iter() {
            if best_moves.contains_key(&stat) {
                continue;
            }
            if stat == forbidden {
                continue;
            }
            best_moves.insert(stat, moves + 1);
            bfs.push_back(stat);
        }
    }
    None
}
