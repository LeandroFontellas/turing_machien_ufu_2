use menu::menu;
use mt::TuringMachine;
use tape::Tape;
use text_io::read;
use utils::{create_mt, is_mt_acceptable, show_mt_details};

mod factory;
mod menu;
mod mt;
mod tape;
mod transitions;
mod utils;

fn main() {
    let mut turing_machine: TuringMachine;
    let mut tape: Tape;
    let mut path: String = String::from("xx.txt");
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
                    None,
                );
                println!("Cadeia testada: {}", word);
                println!("----------Resultado-----------------");

                execute_non_deterministic_mt(&turing_machine, &mut tape);
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

fn execute_non_deterministic_mt(turing_machine: &TuringMachine, tape: &mut Tape) -> bool {
    loop {
        match turing_machine.get_transition(&tape.state, tape.get_current_symbol()) {
            Some(transitions) => {
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
                            Some(tape.tape.clone()),
                        );

                        let is_walkable = new_tape
                            .move_on_tape(transition.direction.clone(), transition.symbol.clone());

                        if !is_walkable {
                            return is_mt_acceptable(&new_turing_machine, &new_tape);
                        }

                        new_tape.set_state(transition.state.clone());

                        execute_non_deterministic_mt(&new_turing_machine, &mut new_tape);
                    } else if current_transitions.len() == 0 {
                        let is_walkable = tape
                            .move_on_tape(transition.direction.clone(), transition.symbol.clone());

                        if !is_walkable {
                            return is_mt_acceptable(&turing_machine, &tape);
                        }
                        tape.set_state(transition.state.clone());
                    }
                }
            }
            None => {
                return is_mt_acceptable(&turing_machine, &tape);
            }
        }
    }
}

// fn pick_random_transition_and_remove_it(transitions: &mut Vec<Transition>) -> Transition {
//     let random_number = rand::thread_rng().gen_range(0..transitions.len());
//     let result = transitions[random_number].clone();

//     transitions.remove(random_number);
//     result
// }
