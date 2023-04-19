use crate::transitions::Transitions;
pub struct TuringMachine {
    states: Vec<String>,
    alphabet: Vec<char>,
    tape_alphabet: Vec<char>,
    initial_state: String,
    accepted_state: Vec<String>,
    white_symbol: char,
    transitions: Transitions,
}

impl TuringMachine {
    pub fn new() -> Self {
        Self {
            states: Vec::new(),
            alphabet: Vec::new(),
            tape_alphabet: Vec::new(),
            initial_state: String::new(),
            accepted_state: Vec::new(),
            white_symbol: '$',
            transitions: Transitions::new(),
        }
    }
}
