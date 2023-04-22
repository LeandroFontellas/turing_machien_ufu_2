use crate::transitions::{TransitionOutput, Transitions};
use std::fmt;
#[derive(Debug)]
pub struct TuringMachine {
    states: Vec<String>,
    alphabet: Vec<char>,
    tape_alphabet: Vec<char>,
    initial_state: String,
    final_states: Vec<String>,
    white_symbol: char,
    transitions: Transitions,
}

impl TuringMachine {
    pub fn new(
        states: Vec<String>,
        alphabet: Vec<char>,
        tape_alphabet: Vec<char>,
        initial_state: String,
        final_states: Vec<String>,
        white_symbol: char,
        transitions: Transitions,
    ) -> Self {
        Self {
            states,
            alphabet,
            tape_alphabet,
            initial_state,
            final_states,
            white_symbol,
            transitions,
        }
    }

    pub fn get_transition(&self, state: &str, symbol: char) -> Option<&TransitionOutput> {
        let mut state_with_symbol = state.to_string();
        state_with_symbol.push(symbol);
        self.transitions.rules.get(&state_with_symbol)
    }

    pub fn is_acceptable(&self, state: &String) -> bool {
        self.final_states.contains(state)
    }
}

impl fmt::Display for TuringMachine {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "States:{:?}\nAlphabet:{:?}\nTapeAlphabet:{:?}\nInitialState:{}\nFinalStates:{:?}\nWhiteSymbol:{}\nTransitions:{}\n", 
            self.states.iter().map(|x| x.to_string().push(',')), 
            self.alphabet.iter().map(|x| x.to_string().push(',')), 
            self.tape_alphabet.iter().map(|x| x.to_string().push(',')), 
            self.initial_state, 
            self.final_states.iter().map(|x| x.to_string().push(',')), 
            self.white_symbol,
            self.transitions
        )
    }
}
