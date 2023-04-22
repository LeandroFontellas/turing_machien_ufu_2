use std::collections::HashMap;
use std::fmt;
#[derive(Debug)]
pub struct Transitions {
    pub rules: HashMap<String, TransitionOutput>,
}

impl fmt::Display for Transitions {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let hashmap = &self.rules;

        for (key, value) in hashmap {
            writeln!(f, "{:?}:{:?}", key, value).unwrap();
        }

        Ok(())
    }
}

impl Transitions {
    pub fn new() -> Transitions {
        Self {
            rules: HashMap::new(),
        }
    }
}

#[derive(Debug)]
pub struct TransitionOutput {
    state: String,
    symbol: char,
    direction: char,
}

impl TransitionOutput {
    pub fn new(state: String, symbol: char, direction: char) -> Self {
        Self {
            state,
            symbol,
            direction,
        }
    }
}

impl fmt::Display for TransitionOutput {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}:{}:{}", self.state, self.symbol, self.direction)
    }
}
