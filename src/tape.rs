use std::fmt;

trait CircularIndex {
    fn circular_index(&self, index: isize) -> usize;
}

impl<T> CircularIndex for Vec<T> {
    fn circular_index(&self, index: isize) -> usize {
        let len = self.len() as isize;
        (((index % len) + len) % len) as usize
    }
}

#[derive(Debug)]
pub struct Tape {
    pub word: String,
    pub white_symbol: char,
    pub accepted_symbols: Vec<String>,
    pub state: String,
    pub size: usize,
    pub tape: Vec<String>,
    pub position: isize,
}

impl Tape {
    pub fn new(
        word: String,
        white_symbol: char,
        accepted_symbols: Vec<String>,
        state: String,
        position: Option<isize>,
        tape: Option<Vec<String>>,
    ) -> Self {
        let mut new_tape = Self {
            word: word.clone(),
            white_symbol,
            accepted_symbols,
            state,
            tape: tape.unwrap_or(word.chars().into_iter().map(|x| x.to_string()).collect()),
            position: position.unwrap_or(0),
            size: word.len() + 1,
        };

        new_tape.tape.insert(word.len(), white_symbol.to_string());

        new_tape
    }

    pub fn move_on_tape(&mut self, direction: String, symbol: String) -> bool {
        let direction = direction.to_lowercase();

        if direction.contains('r') {
            return Self::move_to_right(self, symbol);
        }
        if direction.contains('l') {
            return Self::move_to_left(self, symbol);
        }

        false
    }

    fn move_to_right(&mut self, symbol: String) -> bool {
        if self.accepted_symbols.contains(&symbol) {
            let index = self.tape.circular_index(self.position as isize);
            self.tape[index] = symbol;
            if self.position < self.size as isize {
                self.position += 1;
                return true;
            }
        }

        false
    }

    fn move_to_left(&mut self, symbol: String) -> bool {
        if self.accepted_symbols.contains(&symbol) {
            let index = self.tape.circular_index(self.position as isize);
            self.tape[index] = symbol;

            self.position -= 1;
            return true;
        }

        false
    }

    pub fn get_current_symbol(&self) -> &str {
        let index = self.tape.circular_index(self.position as isize);
        let result = &self.tape[index];

        result
    }

    pub fn set_state(&mut self, state: String) {
        self.state = state;
    }
}

impl fmt::Display for Tape {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut current_tape = String::new();

        for value in self.tape.iter() {
            current_tape.push_str(value);
        }

        writeln!(f, "{}", current_tape)
    }
}
