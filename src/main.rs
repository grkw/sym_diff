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

fn differentiate(input: &Vec<Term>) -> String {

}

fn parse(input: &str) -> Vec<Term> {
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
    terms
}

#[cfg(test)]
mod tests {
    #[test]
    fn constant() {
        // assert_eq!(main(), 4);
    }
    #[test]
    fn test_println() {
        main();
    }
    #[test]
    fn test_parsing() {
        assert_eq!(parse("3x^2 + 2x + 1"), vec![Term { coefficient: 3.0, exponent: 2 }, Term { coefficient: 2.0, exponent: 1 }, Term { coefficient: 1.0, exponent: 0 }]);
        assert_eq!(parse("-3.83x^2"), vec![Term { coefficient: -3.83, exponent: 2 }]);
        
    }
}