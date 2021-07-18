// The code below is a stub. Just enough to satisfy the compiler.
// In order to pass the tests you can add-to or change any of this code.

#[derive(PartialEq, Debug)]
pub enum Direction {
    North,
    East,
    South,
    West,
}

pub struct Robot {
    x: i32,
    y: i32,
    d: Direction,
}

impl Robot {
    pub fn new(x: i32, y: i32, d: Direction) -> Self {
        Self { x, y, d }
    }

    pub fn turn_right(self) -> Self {
        use Direction::*;
        let d = match self.d {
            North => East,
            East => South,
            South => West,
            West => North,
        };
        Self { d, ..self }
    }

    pub fn turn_left(self) -> Self {
        use Direction::*;
        let d = match self.d {
            North => West,
            East => North,
            South => East,
            West => South,
        };
        Self { d, ..self }
    }

    pub fn advance(self) -> Self {
        use Direction::*;
        let x = match &self.d {
            East => self.x + 1,
            West => self.x - 1,
            _ => self.x,
        };
        let y = match &self.d {
            North => self.y + 1,
            South => self.y - 1,
            _ => self.y,
        };
        Self { x, y, ..self }
    }

    pub fn instructions(self, instructions: &str) -> Self {
        let mut s = self;
        for c in instructions.chars() {
            s = match c {
                'R' => s.turn_right(),
                'L' => s.turn_left(),
                'A' => s.advance(),
                _ => s,
            };
        }
        s
    }

    pub fn position(&self) -> (i32, i32) {
        (self.x, self.y)
    }

    pub fn direction(&self) -> &Direction {
        &self.d
    }
}
