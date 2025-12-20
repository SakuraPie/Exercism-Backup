use std::collections::{HashMap, HashSet};

pub fn solve(input: &str) -> Option<HashMap<char, u8>> {
    let puzzle = Puzzle::parse(input)?;
    let solver = Solver::new(&puzzle);
    solver.solve()
}

struct Puzzle {
    operands: Vec<String>,
    result: String,
}

impl Puzzle {
    fn parse(input: &str) -> Option<Self> {
        let parts: Vec<&str> = input.split("==").collect();
        if parts.len() != 2 {
            return None;
        }

        let left_side = parts[0].trim();
        let result = parts[1].trim();

        let operands: Vec<String> = left_side
            .split('+')
            .map(|s| s.trim().to_string())
            .filter(|s| !s.is_empty())
            .collect();

        if operands.is_empty() {
            return None;
        }

        Some(Self {
            operands,
            result: result.to_string(),
        })
    }

    fn max_len(&self) -> usize {
        self.operands
            .iter()
            .map(|s| s.len())
            .max()
            .unwrap_or(0)
            .max(self.result.len())
    }

    fn letters(&self) -> Vec<char> {
        let mut letters_set = HashSet::new();

        for operand in &self.operands {
            for ch in operand.chars() {
                if ch.is_ascii_alphabetic() {
                    letters_set.insert(ch);
                }
            }
        }

        for ch in self.result.chars() {
            if ch.is_ascii_alphabetic() {
                letters_set.insert(ch);
            }
        }

        let mut letters: Vec<char> = letters_set.into_iter().collect();
        letters.sort();
        letters
    }

    fn first_letters(&self) -> HashSet<char> {
        let mut first_letters = HashSet::new();

        for operand in &self.operands {
            if let Some(first_char) = operand.chars().next() {
                if operand.len() > 1 {
                    first_letters.insert(first_char);
                }
            }
        }

        if let Some(first_char) = self.result.chars().next() {
            if self.result.len() > 1 {
                first_letters.insert(first_char);
            }
        }

        first_letters
    }
}

struct Solver<'a> {
    puzzle: &'a Puzzle,
    letters: Vec<char>,
    first_letters: HashSet<char>,
    max_len: usize,
    assignment: HashMap<char, u8>,
    used_digits: [bool; 10],
}

impl<'a> Solver<'a> {
    fn new(puzzle: &'a Puzzle) -> Self {
        Self {
            puzzle,
            letters: puzzle.letters(),
            first_letters: puzzle.first_letters(),
            max_len: puzzle.max_len(),
            assignment: HashMap::new(),
            used_digits: [false; 10],
        }
    }

    fn solve(mut self) -> Option<HashMap<char, u8>> {
        self.backtrack(0, 0)
    }

    fn backtrack(&mut self, col: usize, carry: u32) -> Option<HashMap<char, u8>> {
        if col >= self.max_len {
            if self.assignment.len() == self.letters.len() && self.verify() {
                return Some(self.assignment.clone());
            }
            return None;
        }

        let current_letters = self.collect_current_letters(col);

        if current_letters.is_empty() {
            return self.process_current_column(col, carry);
        }

        self.assign_current_column_letters(&current_letters, col, carry)
    }

    fn collect_current_letters(&self, col: usize) -> Vec<char> {
        let mut letters = Vec::new();

        for operand in &self.puzzle.operands {
            if col < operand.len() {
                let ch = operand.chars().rev().nth(col).unwrap();
                if !self.assignment.contains_key(&ch) {
                    letters.push(ch);
                }
            }
        }

        if col < self.puzzle.result.len() {
            let ch = self.puzzle.result.chars().rev().nth(col).unwrap();
            if !self.assignment.contains_key(&ch) {
                letters.push(ch);
            }
        }

        letters.sort();
        letters.dedup();
        letters
    }

    fn process_current_column(&mut self, col: usize, carry: u32) -> Option<HashMap<char, u8>> {
        let sum = self.calculate_column_sum(col, carry);

        if col < self.puzzle.result.len() {
            let result_ch = self.puzzle.result.chars().rev().nth(col).unwrap();
            let result_digit = *self.assignment.get(&result_ch).unwrap() as u32;

            if sum % 10 == result_digit {
                self.backtrack(col + 1, sum / 10)
            } else {
                None
            }
        } else if sum == 0 {
            self.backtrack(col + 1, 0)
        } else {
            None
        }
    }

    fn assign_current_column_letters(
        &mut self,
        letters: &[char],
        col: usize,
        carry: u32,
    ) -> Option<HashMap<char, u8>> {
        if letters.is_empty() {
            return self.process_current_column(col, carry);
        }

        let current_letter = letters[0];
        let remaining_letters = &letters[1..];

        for digit in 0..=9 {
            if !self.is_digit_valid(current_letter, digit) {
                continue;
            }

            self.assign_letter(current_letter, digit);

            let result = self.assign_current_column_letters(remaining_letters, col, carry);
            if result.is_some() {
                return result;
            }

            self.unassign_letter(current_letter, digit);
        }

        None
    }

    fn is_digit_valid(&self, letter: char, digit: u8) -> bool {
        !self.used_digits[digit as usize]
            && !(digit == 0 && self.first_letters.contains(&letter))
    }

    fn assign_letter(&mut self, letter: char, digit: u8) {
        self.assignment.insert(letter, digit);
        self.used_digits[digit as usize] = true;
    }

    fn unassign_letter(&mut self, letter: char, digit: u8) {
        self.assignment.remove(&letter);
        self.used_digits[digit as usize] = false;
    }

    fn calculate_column_sum(&self, col: usize, carry: u32) -> u32 {
        let mut sum = carry;

        for operand in &self.puzzle.operands {
            if col < operand.len() {
                let ch = operand.chars().rev().nth(col).unwrap();
                sum += *self.assignment.get(&ch).unwrap() as u32;
            }
        }

        sum
    }

    fn verify(&self) -> bool {
        let sum: u64 = self.puzzle.operands
            .iter()
            .map(|operand| self.word_to_number(operand))
            .sum();

        let result = self.word_to_number(&self.puzzle.result);

        sum == result
    }

    fn word_to_number(&self, word: &str) -> u64 {
        let mut num = 0;
        for ch in word.chars() {
            num = num * 10 + *self.assignment.get(&ch).unwrap() as u64;
        }
        num
    }
}