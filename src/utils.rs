use crate::{factory::Factory, mt::TuringMachine, tape::Tape};

pub fn create_mt(path: String) -> TuringMachine {
    match Factory::new(path) {
        Ok(mt) => mt,
        Err(e) => {
            panic!("{}", e);
        }
    }
}

pub fn is_mt_acceptable(turing_machine: &TuringMachine, tape: &Tape) -> bool {
    if turing_machine.is_acceptable(&tape.state) {
        println!("🚀-CADEIA ACEITA!");
        println!("Fita final: {}", tape);
        return true;
    } else {
        println!("😔-CADEIA REJEITADA!");
        println!("Fita final: {}", tape);
        return false;
    }
}

pub fn show_mt_details(mt: &TuringMachine) -> () {
    print!("{}", mt);
}

// pub fn print_transition_result(transition: &Transition) -> () {
//     println!(
//         "({},{},{})",
//         transition.state, transition.symbol, transition.direction
//     );
// }

// pub fn print_transition_key(tape: &Tape) -> () {
//     print!("({},{})->", tape.state, tape.get_current_symbol());
// }
