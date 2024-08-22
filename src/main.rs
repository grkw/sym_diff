use std::io;
mod state_machine; // Include the state_machine.rs file
use state_machine::{StateMachine, Term}; // Bring StateMachine and Term into scope

fn main() {
    println!("Welcome to the symbolic differentiator for polynomial expressions of one variable!");
    println!("Please enter your function to differentiate:");
    let mut user_input = String::new();

    match io::stdin().read_line(&mut user_input) {
        Ok(_) => {
            let user_input = user_input.trim(); // Remove trailing newline
            let mut state_machine = StateMachine::new();
            state_machine.parse_input(&user_input).unwrap();
            for term in &state_machine.terms {
                dbg!("Coefficient: {}, Exponent: {}", term.coefficient, term.exponent);
            }
            let result = differentiate(&state_machine.terms);
            println!("The derivative of the function is: {}", display_result(&result));
        }
        Err(error) => println!("Error reading input: {}", error),
    }
}

fn differentiate(input: &Vec<Term>) -> Vec<Term> {

    let mut differentiated_terms = Vec::new();

    // Differentiate each term
    for input_term in input {
        if input_term.exponent == 0.0 || input_term.coefficient == 0.0 {
            continue; // Derivative of a constant is zero, derivative of 0*x^n is zero
        }
        let diff_term = Term { coefficient: input_term.coefficient * input_term.exponent, exponent: input_term.exponent - 1.0}; // Chain rule
        differentiated_terms.push(diff_term);
    }
    // Add each term to the result
    let mut result: Vec<Term> = Vec::new();
    for diff_term in differentiated_terms {
        let mut found_like_term = false;
        for result_term in &mut result { // Look for existence of a like term
            if result_term.exponent == diff_term.exponent {
                result_term.coefficient += diff_term.coefficient;
                found_like_term = true;
                break;
            }
        }
        if !found_like_term { // Create new term
            result.push(diff_term.clone());
        }
    }

    // Sort result by decreasing exponent
    result.sort_by(|a, b| b.exponent.partial_cmp(&a.exponent).unwrap());

    if result.is_empty() {
        result.push(Term { coefficient: 0.0, exponent: 0.0 });
    }
    result
}

fn display_result(result: &Vec<Term>) -> String {
    let mut result_string = String::new();
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

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_parsing() {
        let mut state_machine = StateMachine::new();
            
        // Vanilla input
        let vanilla_input = "-3x^2 + 2x^1.5 + 5x^6 + 5.5";
        state_machine.parse_input(&vanilla_input).unwrap();
        assert_eq!(state_machine.terms, vec![
            Term { coefficient: -3.0, exponent: 2.0 }, 
            Term { coefficient: 2.0, exponent: 1.5 }, 
            Term { coefficient: 5.0, exponent: 6.0 },
            Term { coefficient: 5.5, exponent: 0.0 },
        ]);
        
        // Negative coefficient, decimal coefficient, negative exponent, decimal exponent
        // let negative_input = "-3.83x^2.9"; // TODO: fix bug thinks this is 0 exp
        // state_machine.parse_input(&negative_input).unwrap();
        // assert_eq!(state_machine.terms, vec![
        //     Term { coefficient: -3.83, exponent: 2.9 }
        // ]);

        // Zero coefficient, zero exponent // TODO: make this allowed. TODO: Handle x instead of x^1
        // let zero_input = "-0x + 0x^0 - 0";
        // assert!(state_machine.parse_input(&zero_input).is_err());

        // Trying to use * for multiply
        let invalid_chars_input = "2*x";
        assert!(state_machine.parse_input(&invalid_chars_input).is_err());
        
        // // Missing term, incomplete term
        let missing_term_input = "6 ++ x^2";
        assert!(state_machine.parse_input(&missing_term_input).is_err());
        
        let incomplete_term_input = "3x^2 + 2x +";
        assert!(state_machine.parse_input(&incomplete_term_input).is_err());
        
        let invalid_coefficient_input = "3x^2 + 2x 1";
        assert!(state_machine.parse_input(&invalid_coefficient_input).is_err());
        
    }
    #[test]
    fn test_differentiation() {
        // Vanilla input, zero coefficient, multiple terms
        assert_eq!(differentiate(&vec![Term { coefficient: 3.0, exponent: 2.0 }, Term { coefficient: 2.0, exponent: 1.0 }, Term { coefficient: 1.0, exponent: 0.0 }]), vec![Term { coefficient: 6.0, exponent: 1.0 }, Term { coefficient: 2.0, exponent: 0.0 }]);
        // Negative coefficient, decimal coefficient, negative exponent, decimal exponent
        assert_eq!(differentiate(&vec![Term { coefficient: -3.83, exponent: 2.5 }]), vec![Term { coefficient: -9.575, exponent: 1.5 }]);
        // Zero coefficient
        assert_eq!(differentiate(&vec![Term { coefficient: 0.0, exponent: 98.0 }]), vec![Term { coefficient: 0.0, exponent: 0.0 }]);
        // Zero exponent
        assert_eq!(differentiate(&vec![Term { coefficient: 3.0, exponent: 0.0 }]), vec![Term { coefficient: 0.0, exponent: 0.0 }]);
    }
}