#[derive(Debug, PartialEq, Eq)]
pub enum Error {
    NotEnoughPinsLeft,
    GameComplete,
}

pub struct BowlingGame {
    rolls: Vec<u16>,
    current_frame: usize,
    current_roll: usize,
    frame_scores: Vec<u16>,
    game_done: bool,
}

impl BowlingGame {
    pub fn new() -> Self {
        BowlingGame {
            rolls: Vec::new(),
            current_frame: 0,
            current_roll: 0,
            frame_scores: vec![0; 10],
            game_done: false,
        }
    }

    pub fn roll(&mut self, pins: u16) -> Result<(), Error> {
        if self.game_done {
            return Err(Error::GameComplete);
        }

        if pins > 10 {
            return Err(Error::NotEnoughPinsLeft);
        }

        self.rolls.push(pins);
        
        if self.current_frame < 9 {
            if self.current_roll == 0 {
                if pins == 10 {
                    self.current_frame += 1;
                } else {
                    self.current_roll = 1;
                }
            } else {
                let prev = self.rolls[self.rolls.len() - 2];
                if prev + pins > 10 {
                    return Err(Error::NotEnoughPinsLeft);
                }
                self.current_frame += 1;
                self.current_roll = 0;
            }
        } else {
            self.current_roll += 1;
            
            match self.current_roll {
                1 => {
                    if pins == 10 {
                        // 第10轮第一次全中
                    }
                }
                2 => {
                    if self.rolls[self.rolls.len() - 2] == 10 {
                        // 第一次全中，第二次可以是任意值
                    } else {
                        let first = self.rolls[self.rolls.len() - 2];
                        if first + pins > 10 {
                            return Err(Error::NotEnoughPinsLeft);
                        }
                        if first + pins < 10 {
                            self.game_done = true;
                        }
                    }
                }
                3 => {
                    let first = self.rolls[self.rolls.len() - 3];
                    let second = self.rolls[self.rolls.len() - 2];
                    
                    if first == 10 {
                        if second != 10 && second + pins > 10 {
                            return Err(Error::NotEnoughPinsLeft);
                        }
                    }
                    self.game_done = true;
                }
                _ => {
                    self.game_done = true;
                    return Err(Error::GameComplete);
                }
            }
        }

        Ok(())
    }

    pub fn score(&self) -> Option<u16> {
        if !self.game_done {
            return None;
        }

        let mut score = 0;
        let mut i = 0;
        
        for frame in 0..10 {
            if i >= self.rolls.len() {
                return None;
            }
            
            if self.rolls[i] == 10 {
                if frame < 9 {
                    if i + 2 < self.rolls.len() {
                        score += 10 + self.rolls[i + 1] + self.rolls[i + 2];
                    } else {
                        return None;
                    }
                    i += 1;
                } else {
                    while i < self.rolls.len() {
                        score += self.rolls[i];
                        i += 1;
                    }
                }
            } else if i + 1 < self.rolls.len() && self.rolls[i] + self.rolls[i + 1] == 10 {
                if frame < 9 {
                    if i + 2 < self.rolls.len() {
                        score += 10 + self.rolls[i + 2];
                    } else {
                        return None;
                    }
                    i += 2;
                } else {
                    while i < self.rolls.len() {
                        score += self.rolls[i];
                        i += 1;
                    }
                }
            } else {
                if i + 1 < self.rolls.len() {
                    score += self.rolls[i] + self.rolls[i + 1];
                    i += 2;
                } else {
                    return None;
                }
            }
        }
        
        Some(score)
    }
}