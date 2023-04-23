use crate::transitions::{TransitionOutput, Transitions};
use std::fmt;
#[derive(Debug)]
pub struct TuringMachine {
    pub states: Vec<String>,
    pub alphabet: Vec<String>,
    pub tape_alphabet: Vec<String>,
    pub initial_state: String,
    pub final_states: Vec<String>,
    pub white_symbol: char,
    pub transitions: Transitions,
}

impl TuringMachine {
    pub fn new(
        states: Vec<String>,
        alphabet: Vec<String>,
        tape_alphabet: Vec<String>,
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

    pub fn get_transition(&self, state: &str, symbol: &str) -> Option<&TransitionOutput> {
        let mut state_with_symbol =  state.to_string();
        state_with_symbol.push_str(symbol);
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
