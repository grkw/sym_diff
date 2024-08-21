use std::io;

#[derive(Debug)]
struct Term { // assumes all polynomials are in the form of a*x^n
    coefficient: f64,
    exponent: f64, // non-fractional exponents only (but they can be negative)
}

fn main() {
    println!("Welcome to the symbolic differentiator for polynomial expressions of one variable!");
    println!("Please enter your function to differentiate:");

    let mut input = String::new();
    match io::stdin().readline(&mut input) {
        Ok(_) => {
            let result = differentiate(&input);
            println!("The derivative of the function is: {}", result);
        }
        Err(error) => println!("Error reading input: {}", error),
    }
}

fn differentiate(input: &Vec<Term>) -> Vec<Term> {
    // Differentiate each term
    for term in input {
        term.coefficient *= term.exponent;
        term.exponent -= 1.0;
    }
    // Add each term to the result
    let mut result = Vec::new();
    for term in input {
        let mut found_like_term = false;
        for result_term in &mut result { // Look for existence of a like term
            if result_term.exponent == term.exponent {
                result_term.coefficient += term.coefficient;
                found_like_term = true;
                break;
            }
        }
        if !found_like_term { // Create new term
            result.push(term.clone());
        }
    }
    result
}

fn display_result(result: &Vec<Term>) -> String {
    let mut result_string = String::new();
    for term in result {
        if term.coefficient != 0.0 {
            if term.coefficient > 0.0 {
                result_string.push('+');
            } else {
                result_string.push('-');
            }
            result_string.push_str(&term.coefficient.to_string());
            if term.exponent != 0.0 {
                result_string.push_str(&format!("x^{}", term.exponent));
            }
        }
    }
    result_string
}

fn parse(input: &str) -> Result<Vec<Term>, String> {

    // Filter out anything that's not a number, x, +, -, ^, ., or whitespace
    for c in input.chars() {
        if !c.is_numeric() && c != 'x' && c != '+' && c != '-' && c != '^' && c != '.' && c != ' ' {
            return Err("Invalid characters".to_string());
        }
    }

    let mut terms = Vec::new();
    let mut term = Term { coefficient: 0.0, exponent: 0 }; // In case nothing is entered
    let mut coefficient = String::new();
    let mut exponent = String::new();
    let mut is_coefficient = true;
    for c in input.chars() {
        match c {
            ' ' => continue,
            '+' | '-' => {
                if !coefficient.is_empty() {
                    term.coefficient = coefficient.parse().unwrap();
                    term.exponent = exponent.parse().unwrap();
                    terms.push(term);
                    term = Term { coefficient: 0.0, exponent: 0 };
                    coefficient.clear();
                    exponent.clear();
                    is_coefficient = true;
                }
                if c == '-' {
                    coefficient.push(c);
                }
            }
            'x' => {
                is_coefficient = false;
            }
            '^' => {
                is_coefficient = false;
            }
            _ => {
                if is_coefficient {
                    coefficient.push(c);
                } else {
                    exponent.push(c);
                }
            }
        }
    }
    if !coefficient.is_empty() {
        term.coefficient = coefficient.parse().unwrap();
        term.exponent = exponent.parse().unwrap();
        terms.push(term);
    }
    Ok(terms)
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_parsing() {
        // Vanilla input
        assert_eq!(parse("3x^2+ 2x +1"), vec![Term { coefficient: 3.0, exponent: 2 }, Term { coefficient: 2.0, exponent: 1 }, Term { coefficient: 1.0, exponent: 0 }]);
        // Negative coefficient, decimal coefficient, negative exponent, decimal exponent
        assert_eq!(parse("-3.83x^2.9"), vec![Term { coefficient: -3.83, exponent: 2 }]);
        // Trying to use * for multiply
        assert_eq!(parse("2*x"), Err("Invalid characters".to_string()));
        // Missing term, incomplete term
        assert_eq!(parse("6 ++ x^2"), Err("Invalid input".to_string()));
        assert_eq!(parse("3x^2 + 2x +"), Err("Invalid input".to_string()));
        assert_eq!(parse("3x^2 + 2x 1"), Err("Invalid input".to_string()));
        // Zero coefficient, zero exponent
        assert_eq!(parse("-0x + 0x^0 - 0"), Term { coefficient: 0.0, exponent: 0 });
    }
    #[test]
    fn test_differentiation() {
        // Vanilla input
        assert_eq!(differentiate(&vec![Term { coefficient: 3.0, exponent: 2 }, Term { coefficient: 2.0, exponent: 1 }, Term { coefficient: 1.0, exponent: 0 }]), vec![Term { coefficient: 6.0, exponent: 1 }, Term { coefficient: 2.0, exponent: 0 }]);
        // Negative coefficient, decimal coefficient, negative exponent, decimal exponent
        assert_eq!(differentiate(&vec![Term { coefficient: -3.83, exponent: 2 }]), vec![Term { coefficient: -7.66, exponent: 1 }]);
        // Zero coefficient, zero exponent
        assert_eq!(differentiate(&vec![Term { coefficient: 0.0, exponent: 0 }]), vec![Term { coefficient: 0.0, exponent: 0 }]);
    }
}