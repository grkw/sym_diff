use crate::state_machine::Term;

pub fn differentiate(input: &Vec<Term>) -> Vec<Term> {

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

#[cfg(test)]
mod differentiate_tests {
    use super::*;
    #[test]
    fn test_differentiation_vanilla_input() {
        assert_eq!(differentiate(&vec![
            Term { coefficient: 3.0, exponent: -2.0 }, 
            Term { coefficient: -2.0, exponent: 1.0 }, 
            Term { coefficient: -1.0, exponent: 0.0 }
        ]), vec![
            Term { coefficient: -2.0, exponent: 0.0 },
            Term { coefficient: -6.0, exponent: -3.0 }, 
        ]);
    }

    #[test]
    fn test_differentiation_negative_input() {
        assert_eq!(differentiate(&vec![
            Term { coefficient: -3.83, exponent: 2.5 }
        ]), vec![
            Term { coefficient: -9.575, exponent: 1.5 }
        ]);
    }

    #[test]
    fn test_differentiation_zero_coefficient() {
        assert_eq!(differentiate(&vec![
            Term { coefficient: 0.0, exponent: 98.0 }
        ]), vec![
            Term { coefficient: 0.0, exponent: 0.0 }
        ]);
    }

    #[test]
    fn test_differentiation_zero_exponent() {
        assert_eq!(differentiate(&vec![
            Term { coefficient: 3.0, exponent: 0.0 }
        ]), vec![
            Term { coefficient: 0.0, exponent: 0.0 }
        ]);
    }
}