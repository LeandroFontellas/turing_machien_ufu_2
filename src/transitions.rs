use std::collections::HashMap;
use std::fmt;
#[derive(Debug)]
pub struct Transitions {
    pub rules: HashMap<String, Vec<Transition>>,
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

impl Clone for Transitions {
    fn clone(&self) -> Transitions {
        let mut new_hash_map: HashMap<String, Vec<Transition>> = HashMap::new();
        for (key, transitions) in &self.rules {
            for transition in transitions {
                new_hash_map
                    .entry(key.to_string())
                    .and_modify(|x| {
                        x.push(transition.clone());
                    })
                    .or_insert({
                        let mut vec = Vec::new();
                        vec.push(transition.clone());
                        vec
                    });
            }
        }
        Self {
            rules: new_hash_map,
        }
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
pub struct Transition {
    pub state: String,
    pub symbol: String,
    pub direction: String,
}

impl Clone for Transition {
    fn clone(&self) -> Transition {
        Self {
            direction: self.direction.clone(),
            state: self.state.clone(),
            symbol: self.symbol.clone(),
        }
    }
}

impl Transition {
    pub fn new(state: String, symbol: String, direction: String) -> Self {
        Self {
            state,
            symbol,
            direction,
        }
    }
}

impl fmt::Display for Transition {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}:{}:{}", self.state, self.symbol, self.direction)
    }
}
