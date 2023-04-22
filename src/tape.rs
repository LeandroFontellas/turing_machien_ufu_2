use std::fmt;

#[derive(Debug)]
pub struct Tape {
    word: String,
    white_symbol: char,
    accepted_symbols: Vec<char>,
    state: String,
    size: usize,
    tape: Vec<char>,
    position: usize,
}

impl Tape {
    pub fn new(
        word: String,
        white_symbol: char,
        accepted_symbols: Vec<char>,
        state: String,
    ) -> Self {
        let mut new_tape = Self {
            word: word.clone(),
            white_symbol,
            accepted_symbols,
            state,
            tape: word.chars().into_iter().collect(),
            position: 1,
            size: word.len() + 1,
        };

        new_tape.tape.insert(word.len() - 1, white_symbol);

        new_tape
    }

    pub fn move_on_tape(&mut self, direction: char, symbol: char) -> bool {
        let mut direction = direction.to_lowercase();

        if direction.any(|x| x == 'r') {
            return Self::move_to_right(self, symbol);
        }
        if direction.any(|x| x == 'l') {
            return Self::move_to_left(self, symbol);
        }

        false
    }

    fn move_to_right(&mut self, symbol: char) -> bool {
        if self.accepted_symbols.contains(&symbol) {
            self.tape[self.position] = symbol;
            if self.position < self.size {
                self.position += 1;
                return true;
            }
        }

        false
    }

    fn move_to_left(&mut self, symbol: char) -> bool {
        if self.accepted_symbols.contains(&symbol) {
            self.tape[self.position] = symbol;

            self.position -= 1;
            return true;
        }

        false
    }

    pub fn get_current_symbol(&self) -> char {
        self.tape[self.position]
    }
}

impl fmt::Display for Tape {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut current_tape = String::new();

        for (index, value) in self.tape.iter().enumerate() {
            if index == self.position {
                current_tape.push_str(&self.state);
            }
            current_tape.push(*value);
        }

        writeln!(f, "{}", current_tape)
    }
}
