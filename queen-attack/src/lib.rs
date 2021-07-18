#[derive(Debug)]
pub struct ChessPosition {
    rank: i32,
    file: i32,
}

#[derive(Debug)]
pub struct Queen {
    position: ChessPosition,
}

const BOARD_SIZE: i32 = 8;
impl ChessPosition {
    pub fn new(rank: i32, file: i32) -> Option<Self> {
        if !(0..BOARD_SIZE).contains(&rank) || !(0..BOARD_SIZE).contains(&file) {
            None
        } else {
            Some(Self { rank, file })
        }
    }
}

impl Queen {
    pub fn new(position: ChessPosition) -> Self {
        Self { position }
    }

    pub fn can_attack(&self, other: &Queen) -> bool {
        let lhs = &self.position;
        let rhs = &other.position;
        lhs.rank == rhs.rank
            || lhs.file == rhs.file
            || (lhs.rank - rhs.rank).abs() == (lhs.file - rhs.file).abs()
    }
}
