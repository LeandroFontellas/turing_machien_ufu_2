use menu::menu;
use mt::TuringMachine;
use tape::Tape;
use text_io::read;
use transitions::Transition;

use crate::factory::Factory;

mod factory;
mod menu;
mod mt;
mod tape;
mod transitions;
fn main() {
    let mut turing_machine: TuringMachine;
    let mut tape: Tape;
    let mut path: String = String::from("anbncn.txt");
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
                );
                println!("Cadeia testada: {}", word);
                println!("----------Resultado-----------------");

                execute_mt(turing_machine, tape, 0);

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

fn create_mt(path: String) -> TuringMachine {
    match Factory::new(path) {
        Ok(mt) => mt,
        Err(e) => {
            panic!("{}", e);
        }
    }
}

fn is_mt_acceptable(turing_machine: &TuringMachine, tape: &Tape) -> bool {
    if turing_machine.is_acceptable(&tape.state) {
        println!("CADEIA ACEITA!");
        return true;
    } else {
        println!("CADEIA REJEITADA!");
        return false;
    }
}

fn show_mt_details(mt: &TuringMachine) -> () {
    print!("{}", mt);
}

fn execute_mt(turing_machine: TuringMachine, tape: Tape, n: u32) -> bool {
    loop {
        match turing_machine.get_transition(&tape.state, tape.get_current_symbol()) {
            Some(transition) => {
                print!("({},{})->", tape.state, tape.get_current_symbol());

                let is_walkable = walk(transition, turing_machine, tape, 1);

                if !is_walkable {
                    return is_mt_acceptable(&turing_machine, &tape);
                }

                print_transition_result(transition);
            }
            None => {
                return is_mt_acceptable(&turing_machine, &tape);
            }
        }
    }
}

// fn execute(turing_machine: &TuringMachine, tape: &mut Tape, n: u32) -> bool {
//     let transitions = turing_machine.get_transition(&tape.state, tape.get_current_symbol());

//     if n > 1000 {
//         return false;
//     }

//     match transitions {
//         Some(t_vec) => {
//             if t_vec.len() > 1 {
//                 for _transition in t_vec {
//                     return execute(turing_machine.clone(), tape, n + 1);
//                 }
//             } else if t_vec.len() == 1 {
//                 return execute_mt(turing_machine, tape, n + 1);
//             }
//         }
//         None => return is_mt_acceptable(turing_machine, tape),
//     }

//     false
// }

fn walk(transitions: &Vec<Transition>, turing_machine: TuringMachine, tape: Tape, n: u32) -> bool {
    if n > 1000 {
        return false;
    }
    /* Ele aqui está fazendo um movimento da cabeça para cada transição com a mesma chave
    não queremos isso, queremos que ele crie uma maquina de turing igual porém a diferença é que
    as maquinas vão ter tapes diferentes pq eles vão escolher caminhos diferentes,
    no final ele deve retornar o resultado true or false*/
    if transitions.len() > 1 {
        for transition in transitions {
            let new_turing_machine = TuringMachine::new(
                turing_machine.states.clone(),
                turing_machine.alphabet.clone(),
                turing_machine.tape_alphabet.clone(),
                turing_machine.initial_state.clone(),
                turing_machine.final_states.clone(),
                turing_machine.white_symbol,
                turing_machine.transitions,
            );
            // a nova tape precisa ter estado inicial como sendo o estado atual da tape antiga?
            // antes ou depois de realizar o movimento da cabeca?
            let new_tape = Tape::new(
                tape.word,
                turing_machine.white_symbol,
                turing_machine.tape_alphabet.clone(),
                tape.state,
            );

            let is_walkable =
                new_tape.move_on_tape(transition.direction.clone(), transition.symbol.clone());

            new_tape.set_state(transition.state.clone());

            return execute_mt(new_turing_machine, tape, n + 1);
        }
    } else if transitions.len() == 1 {
        return execute_mt(turing_machine, tape, n + 1);
    }

    false
}

fn print_transition_result(transition: &Vec<Transition>) -> () {
    println!(
        "({},{},{})",
        transition.state, transition.symbol, transition.direction
    );
}
