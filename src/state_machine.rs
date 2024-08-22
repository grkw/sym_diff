#[derive(Debug, PartialEq, Clone)]
pub struct Term { // assumes all polynomials are in the form of a*x^n
    pub coefficient: f64,
    pub exponent: f64, // non-fractional exponents only (but they can be negative)
}

pub enum State {
    ParseCoefficient,
    ParseCaret,
    ParseExponent,
}

pub struct StateMachine {
    state: State,
    new_coeff: String,
    new_exp: String,
    pub terms: Vec<Term>,
    new_term: Term,
}

impl StateMachine {
    pub fn new() -> Self { // constructor
        StateMachine {
            state: State::ParseCoefficient,
            new_coeff: String::new(),
            new_exp: String::new(),
            terms: Vec::new(),
            new_term: Term { coefficient: 0.0, exponent: 0.0 },
        }
    }

    pub fn handle_event(&mut self, event: char) -> Result<(), String> {
        match self.state {
            State::ParseCoefficient => match event {

                '0'..='9' | '+' | '-' | '.' => { // String->f64 parsing works with +/-/.
                    // println!("ParseCoefficient: {}", event);
                    self.new_coeff.push(event);
                    Ok(())
                }
                'x' => { // Done getting the coefficient
                    // println!("ParseCoefficient: {}", event);
                    self.state = State::ParseCaret;
                    Ok(())
                }
                _ => Err(format!("Invalid input: {}", event)),
            },
            State::ParseCaret => match event {
                '^' => {
                    // println!("ParseCaret: {}", event);
                    self.state = State::ParseExponent; 
                    Ok(())
                }
                _ => Err(format!("Invalid input: {}", event)),
            }
            State::ParseExponent => match event {
                '0'..='9' | '.' => {
                    self.new_exp.push(event);
                    Ok(())
                },
                '+' | '-' => {
                    // Done getting this term
                    self.new_term.coefficient = self.new_coeff.parse().unwrap();
                    self.new_term.exponent = self.new_exp.parse().unwrap();
                    self.terms.push(self.new_term.clone());
                    
                    // Reset for the next term
                    self.new_term = Term { coefficient: 0.0, exponent: 0.0 };
                    self.new_coeff.clear();
                    self.new_exp.clear();
                    self.state = State::ParseCoefficient;
                    if event == '-' { // Need to explicitly push the negative sign for the coeff
                        self.new_coeff.push('-');
                    }
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

        for c in input.chars() {
            println!("c: {}", c);
            if c == ' ' {
                continue;
            }
            if let Err(e) = self.handle_event(c) {
                eprintln!("Error handling event '{}': {}", c, e);
                break;
            }
        }
        if !self.new_coeff.is_empty() {
            self.new_term.coefficient = self.new_coeff.parse().unwrap();
            self.new_term.exponent = if self.new_exp.is_empty() { 0.0 } else { self.new_exp.parse().unwrap() };
            self.terms.push(self.new_term.clone());
        }
        Ok(())
    }

}