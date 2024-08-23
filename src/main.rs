use std::io;

mod state_machine; // Include the state_machine.rs file
mod differentiate; // Include the differentiate.rs file

use crate::state_machine::{StateMachine, Term}; // Bring StateMachine and Term into scope
use crate::differentiate::differentiate;

fn main() {
    println!("Welcome to the symbolic differentiator for polynomial expressions of one variable!");
    println!("Please enter your function to differentiate:");
    let mut user_input = String::new();

    match io::stdin().read_line(&mut user_input) {
        Ok(_) => {
            let mut state_machine = StateMachine::new();
            state_machine.parse_input(&user_input).unwrap();
            let result = differentiate(&state_machine.terms);
            println!("The derivative of the function is: {}", display_result(&result));
        }
        Err(error) => println!("Error reading input: {}", error),
    }
}

fn display_result(result: &Vec<Term>) -> String {
    let mut result_string = String::new();
    if result.is_empty() || result.len() == 1 && result[0].coefficient == 0.0 {
        return "0".to_string();
    }
    for term in result {
        if term.coefficient != 0.0 {
            if term.coefficient > 0.0 { // Need to explicitly display the positive sign for the coeff
                result_string.push('+');
            }
            result_string.push_str(&term.coefficient.to_string());
            if term.exponent != 0.0 {
                result_string.push_str(&format!("x^{}", term.exponent));
            }
        }
    }
    result_string
}