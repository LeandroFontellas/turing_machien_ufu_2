use crate::mt::TuringMachine;
use crate::transitions::{Transition, Transitions};
use std::fs::File;
use std::io::{self, BufRead};
#[derive(Debug)]
pub struct Factory {}

const INNER_SPLITTER: char = ',';
const OUTER_SPLITTER: char = ';';

impl Factory {
    pub fn new(filename: String) -> Result<TuringMachine, &'static str> {
        let (states, alphabet, tape_alphabet, initial_state, final_states, white_symbol): (
            Vec<String>,
            Vec<String>,
            Vec<String>,
            String,
            Vec<String>,
            char,
        );

        match File::open(filename) {
            Ok(file) => {
                let mut reader = io::BufReader::new(file).lines();
                let result = reader.next().expect("States to be readable");

                states = result
                    .unwrap()
                    .split(OUTER_SPLITTER)
                    .map(|x| x.to_string())
                    .collect();

                let result = reader.next().expect("Alphabet to be readable");
                alphabet = result
                    .unwrap()
                    .split(OUTER_SPLITTER)
                    .map(|x| x.to_string())
                    .collect();

                let result = reader.next().expect("TapeAlphabet to be readable");
                tape_alphabet = result
                    .unwrap()
                    .split(OUTER_SPLITTER)
                    .map(|x| x.to_string())
                    .collect();

                let result = reader.next().expect("Tape Alphabet to be readable");
                initial_state = result.unwrap();

                let result = reader.next().expect("Final States to be readable");
                final_states = result
                    .unwrap()
                    .split(OUTER_SPLITTER)
                    .map(|x| x.to_string())
                    .collect();

                let result = reader.next().expect("White symbol to be readable");
                white_symbol = result.unwrap().chars().collect::<Vec<char>>()[0];

                let result = reader.next().expect("Transitions to be readable");
                let stringfied_transitions = result
                    .unwrap()
                    .split(OUTER_SPLITTER)
                    .map(|x| x.to_string())
                    .collect::<Vec<String>>();
                let mut transitions = Transitions::new();
                for val in stringfied_transitions {
                    let splitted_value = val.split("->").collect::<Vec<&str>>();
                    let entry = Self::create_rule_key(splitted_value[0]);

                    transitions
                        .rules
                        .entry(entry)
                        .and_modify(|x| x.push(Self::strip_transition(splitted_value[1])))
                        .or_insert(Self::create_transition_output(splitted_value[1]));
                }
                Ok(TuringMachine::new(
                    states,
                    alphabet,
                    tape_alphabet,
                    initial_state,
                    final_states,
                    white_symbol,
                    transitions,
                ))
            }
            Err(_) => Err("Failed to open the file"),
        }
    }

    fn create_rule_key(val: &str) -> String {
        let result = val
            .strip_prefix("(")
            .expect("To have (")
            .strip_suffix(")")
            .expect("To have )")
            .split(",")
            .collect();

        result
    }

    fn create_transition_output(val: &str) -> Vec<Transition> {
        vec![Self::strip_transition(val)]
    }

    fn strip_transition(val: &str) -> Transition {
        let stripped_value: Vec<&str> = val
            .strip_prefix("(")
            .expect("To have (")
            .strip_suffix(")")
            .expect("To have )")
            .split(INNER_SPLITTER)
            .collect();

        Transition::new(
            stripped_value[0].to_string(),
            stripped_value[1].to_string(),
            stripped_value[2].to_string(),
        )
    }
}
