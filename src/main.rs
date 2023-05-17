use menu::menu;
use mt::TuringMachine;
use rand::prelude::*;
use tape::Tape;
use text_io::read;
use transitions::Transition;
use utils::{
    create_mt, is_mt_acceptable, print_transition_key, print_transition_result, show_mt_details,
};

mod factory;
mod menu;
mod mt;
mod tape;
mod transitions;
mod utils;

fn main() {
    let mut turing_machine: TuringMachine;
    let mut tape: Tape;
    let mut path: String = String::from("nondeterministicmt2.txt");
    // create default mt
    turing_machine = create_mt(path);
    show_mt_details(&turing_machine);
    menu();
    let mut option: u8 = read!();

    while option != 3 {
        match option {
            1 => {
                println!("Digite o nome do arquivo: ");
                path = read!();
                turing_machine = create_mt(path);
                show_mt_details(&turing_machine);
            }
            2 => {
                println!("Digite uma cadeia:");
                let word: String = read!();
                tape = Tape::new(
                    word.clone(),
                    turing_machine.white_symbol,
                    turing_machine.tape_alphabet.clone(),
                    turing_machine.initial_state.clone(),
                    None,
                );
                println!("Cadeia testada: {}", word);
                println!("----------Resultado-----------------");

                execute_non_deterministic_mt(&turing_machine, &mut tape, 1);

                println!("Fita final: {}", tape);
            }
            _ => {
                println!("Fim");
                break;
            }
        }
        println!("Digite uma opção novamente: ");
        option = read!();
    }
}

fn execute_non_deterministic_mt(turing_machine: &TuringMachine, tape: &mut Tape, n: u32) -> bool {
    let mut final_result = false;
    loop {
        match turing_machine.get_transition(&tape.state, tape.get_current_symbol()) {
            Some(transitions) => {
                // print_transition_key(tape);

                if n > 1000 {
                    return false;
                }

                // trabalha em cima de uma copia para nao alterar as definicoes
                let mut current_transitions = transitions.clone();

                for _tr in current_transitions.clone() {
                    let transition = current_transitions
                        .pop()
                        .expect("To have at least 1 transition");

                    if current_transitions.len() >= 1 {
                        let new_turing_machine = turing_machine.clone();

                        let mut new_tape = Tape::new(
                            tape.word.clone(),
                            turing_machine.white_symbol,
                            turing_machine.tape_alphabet.clone(),
                            tape.state.clone(), // estado inicial é o estado atual da ultima fita
                            Some(tape.position), // inicia na posicao da ultima fita
                        );

                        let is_walkable = new_tape
                            .move_on_tape(transition.direction.clone(), transition.symbol.clone());

                        if !is_walkable {
                            final_result =
                                final_result || is_mt_acceptable(&new_turing_machine, &new_tape);
                            return final_result;
                        }

                        new_tape.set_state(transition.state.clone());

                        final_result = final_result
                            || execute_non_deterministic_mt(
                                &new_turing_machine,
                                &mut new_tape,
                                n + 1,
                            );
                    } else if current_transitions.len() == 0 {
                        let is_walkable = tape
                            .move_on_tape(transition.direction.clone(), transition.symbol.clone());
                        tape.set_state(transition.state.clone());

                        if !is_walkable {
                            return is_mt_acceptable(&turing_machine, &tape);
                        }

                        // print_transition_result(&transition);
                    }
                }
            }
            None => {
                return is_mt_acceptable(&turing_machine, &tape);
            }
        }
    }
}

/**
 * Modifies the transitions vector
 * returns the random picked transition
 */
fn pick_random_transition_and_remove_it(transitions: &mut Vec<Transition>) -> Transition {
    let random_number = rand::thread_rng().gen_range(0..transitions.len());
    let result = transitions[random_number].clone();

    transitions.remove(random_number);
    result
}
