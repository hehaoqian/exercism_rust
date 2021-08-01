#![allow(clippy::new_without_default)]
#[derive(Debug, PartialEq)]
pub enum Error {
    NotEnoughPinsLeft,
    GameComplete,
}

const TOTAL_FRAMES: u8 = 10;
const TOTAL_PINS: u8 = 10;
const THROW_CHANCE_PER_FRAME: u8 = 2;

#[derive(PartialEq)]
enum FrameCategory {
    Strike,
    Sparse,
    Open,
}

fn get_frame_category(frame: u8, history: &[u8]) -> FrameCategory {
    use FrameCategory::*;
    if frame == TOTAL_FRAMES - 1 {
        Open
    } else if history.first().cloned().unwrap() == TOTAL_PINS {
        Strike
    } else if history.iter().cloned().sum::<u8>() == TOTAL_PINS {
        Sparse
    } else {
        Open
    }
}

pub struct BowlingGame {
    frame: u8,
    frame_pins_left: u8,
    frame_pins_down: u8,
    frame_throw_left: u8,
    frame_throw_done: u8,
    frame_first_throw_down: u8,
    frame_get_fill_ball: bool,
    frame_history: Vec<Vec<u8>>,
}

impl BowlingGame {
    pub fn new() -> Self {
        Self {
            frame: 0,
            frame_pins_left: TOTAL_FRAMES,
            frame_throw_left: THROW_CHANCE_PER_FRAME,
            frame_pins_down: 0,
            frame_throw_done: 0,
            frame_first_throw_down: 0,
            frame_get_fill_ball: false,
            frame_history: vec![vec![]],
        }
    }

    fn completed(&self) -> bool {
        self.frame >= TOTAL_FRAMES
    }

    fn prepare_next_frame(&mut self) {
        self.frame += 1;
        self.frame_pins_left = TOTAL_PINS;
        self.frame_pins_down = 0;
        self.frame_throw_left = THROW_CHANCE_PER_FRAME;
        self.frame_throw_done = 0;
        self.frame_first_throw_down = 0;
        if !self.completed() {
            self.frame_history.push(Vec::new());
        }
    }

    fn is_last_frame(&self) -> bool {
        self.frame == TOTAL_FRAMES - 1
    }

    pub fn roll(&mut self, pins: u16) -> Result<(), Error> {
        if self.completed() {
            return Err(Error::GameComplete);
        }
        if pins > self.frame_pins_left as u16 {
            return Err(Error::NotEnoughPinsLeft);
        }
        self.frame_pins_left -= pins as u8;
        self.frame_pins_down += pins as u8;
        self.frame_throw_left -= 1;
        self.frame_throw_done += 1;

        if self.is_last_frame() && self.frame_pins_left == 0 {
            if !self.frame_get_fill_ball {
                self.frame_throw_left += 1;
                self.frame_pins_left = TOTAL_PINS;
                self.frame_get_fill_ball = true;
            }
            self.frame_pins_left = 10;
        }

        self.frame_history.last_mut().unwrap().push(pins as u8);
        if self.frame_throw_left == 0 || self.frame_pins_left == 0 {
            self.prepare_next_frame();
        }
        Ok(())
    }

    fn hits_single_throw(&self, index: u8) -> Option<u8> {
        self.frame_history
            .iter()
            .flatten()
            .nth(index as usize)
            .cloned()
    }

    pub fn score(&self) -> Option<u16> {
        if self.completed() {
            let mut num_throws = 0;
            let mut scores: u16 = 0;
            for (frame, history) in self.frame_history.iter().enumerate() {
                num_throws += history.len() as u8;
                use FrameCategory::*;
                match get_frame_category(frame as u8, history) {
                    Strike => {
                        scores += (TOTAL_PINS
                            + self.hits_single_throw(num_throws).unwrap_or(0)
                            + self.hits_single_throw(num_throws + 1).unwrap_or(0))
                            as u16
                    }
                    Sparse => {
                        scores +=
                            (TOTAL_PINS + self.hits_single_throw(num_throws).unwrap_or(0)) as u16
                    }
                    Open => scores += history.iter().sum::<u8>() as u16,
                }
            }
            Some(scores)
        } else {
            None
        }
    }
}
