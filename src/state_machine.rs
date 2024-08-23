#[derive(Debug, PartialEq, Clone)]
pub struct Term {
    pub coefficient: f64, // can be negative, decimal
    pub exponent: f64, // can be decimal, but not negative
}

#[derive(Debug, PartialEq)]
pub enum State {
    ParseCoefficient,
    ParseCaret,
    ParseExponent,
}

pub struct StateMachine {
    state: State,
    pub terms: Vec<Term>,
    new_coeff: String,
    new_exp: String,
    new_term: Term,
}

impl StateMachine {
    pub fn new() -> Self { // constructor
        StateMachine {
            state: State::ParseCoefficient,
            terms: Vec::new(),
            new_coeff: String::new(), // Strings as we're building up the coefficient and exponent...
            new_exp: String::new(),
            new_term: Term { coefficient: 0.0, exponent: 0.0 }, // ...once they're built, parse and insert them here
        }
    }

    pub fn handle_event(&mut self, event: char) -> Result<(), String> {
        #[cfg(test)] {
            println!("State: {:?}, Event: {}, New coeff: {}, New exp: {}", self.state, event, self.new_coeff, self.new_exp);
        }
        match self.state {
            State::ParseCoefficient => match event {

                '0'..='9' | '+' | '-' | '.' => { // String->f64 parsing works with +/-/.
                    self.new_coeff.push(event);
                    Ok(())
                }
                'x' => { // Done getting the coefficient
                    self.state = State::ParseCaret;
                    Ok(())
                }
                '\n' => Ok(()), // ending on a coefficient is valid
                _ => Err(format!("Invalid input: {}", event)),
            },
            State::ParseCaret => match event {
                '^' => {
                    self.state = State::ParseExponent; 
                    Ok(())
                }
                '+' | '-' => {
                    // Done getting this term
                    self.new_term.exponent = 1.0;
                    println!("new coeff: {}", self.new_coeff);
                    self.new_term.coefficient = self.new_coeff.parse().map_err(|e: std::num::ParseFloatError| e.to_string())?;
                    self.terms.push(self.new_term.clone());
                    
                    self.new_coeff.clear();
                    if event == '-' { // Need to explicitly push the negative sign for the coeff
                        self.new_coeff.push('-');
                    }
                    self.new_term = Term { coefficient: 0.0, exponent: 0.0 };
                    self.new_exp.clear();
                    self.state = State::ParseCoefficient;
                    Ok(())
                }
                _ => Err(format!("Invalid input: {}", event)),
            }
            State::ParseExponent => match event {
                '0'..='9' | '.' => {
                    self.new_exp.push(event);
                    Ok(())
                },
                '+' | '-' | '\n' => {
                    self.new_term.coefficient = self.new_coeff.parse().map_err(|e: std::num::ParseFloatError| e.to_string())?;
                    self.new_term.exponent = self.new_exp.parse().map_err(|e: std::num::ParseFloatError| e.to_string())?;
                    self.terms.push(self.new_term.clone());

                    self.new_coeff.clear();
                    if event == '-' { // Need to explicitly push the negative sign for the coeff
                        self.new_coeff.push('-');
                    }
                    self.new_term = Term { coefficient: 0.0, exponent: 0.0 };
                    self.new_exp.clear();
                    // Done getting this term
                    self.state = State::ParseCoefficient;
                    Ok(())
                }
                _ => Err(format!("Invalid input: {}", event)),
            }
        }
    }

    pub fn parse_input(&mut self, input: &str) -> Result<(), String> {
        // Reset the state machine
        self.state = State::ParseCoefficient;
        self.new_coeff.clear();
        self.new_exp.clear();
        self.terms.clear();
        self.new_term = Term { coefficient: 0.0, exponent: 0.0 };

        // Parse the input
        for c in input.chars() {
            if c == ' ' { // ignore whitespace; not valid nor invalid
                continue;
            }
            self.handle_event(c)?;
        }
        if !self.new_coeff.is_empty() { // A coefficient at the end, but no exponent
            self.new_term.coefficient = self.new_coeff.parse().map_err(|e: std::num::ParseFloatError | e.to_string())?;
            self.terms.push(self.new_term.clone());
        }
        Ok(())
    }
}

#[cfg(test)]
mod parse_tests {
    use super::*;
    #[test]
    fn test_parse_vanilla_input() {
        let mut state_machine = StateMachine::new();
        let vanilla_input = "-3x^2 + 4x^1.5 + 7x^6 + 8.9\n";
        state_machine.parse_input(&vanilla_input).unwrap();
        assert_eq!(state_machine.terms, vec![
            Term { coefficient: -3.0, exponent: 2.0 }, 
            Term { coefficient: 4.0, exponent: 1.5 }, 
            Term { coefficient: 7.0, exponent: 6.0 },
            Term { coefficient: 8.9, exponent: 0.0 },
        ]);
    }

    #[test]
    fn test_parse_negative_input() {
        let mut state_machine = StateMachine::new();
        let negative_input = "-3.83x^2.9\n";
        state_machine.parse_input(&negative_input).unwrap();
        assert_eq!(state_machine.terms, vec![
            Term { coefficient: -3.83, exponent: 2.9 }
        ]);
    }

    #[test]
    fn test_parse_incomplete_term_input() { // TODO: Should wait until a digit is actually entered in ParseCoefficient
        let mut state_machine = StateMachine::new();
        let incomplete_term_input = "3x^2 + 2x +\n";
        state_machine.parse_input(&incomplete_term_input).unwrap();
        assert_eq!(state_machine.terms, vec![
            Term { coefficient: 3.0, exponent: 2.0 },
            Term { coefficient: 2.0, exponent: 1.0 }
        ]);
    }

    #[test]
    fn test_parse_invalid_var_input() {
        let mut state_machine = StateMachine::new();
        let invalid_var_input = "3y^2 + 2y^1\n";
        assert!(state_machine.parse_input(&invalid_var_input).is_err());
    }

    #[test]
    fn test_parse_invalid_char_input() {
        let mut state_machine = StateMachine::new();
        let invalid_char_input = "2*x\n";
        assert!(state_machine.parse_input(&invalid_char_input).is_err());
    }

    #[test]
    fn test_parse_missing_term_input() {
        let mut state_machine = StateMachine::new();
        let missing_term_input = "6 ++ x^2\n";
        assert!(state_machine.parse_input(&missing_term_input).is_err());
    }

    #[test]
    fn test_parse_invalid_coefficient_input() {
        let mut state_machine = StateMachine::new();
        let invalid_coefficient_input = "3x^2 + 2x 1\n";
        assert!(state_machine.parse_input(&invalid_coefficient_input).is_err());
    }
}