#[derive(Debug, PartialEq, Clone)]
pub struct Term { // assumes all polynomials are in the form of a*x^n
    pub coefficient: f64, // can be negative, decimal
    pub exponent: f64, // can be decimal, but not negative
}

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
            new_coeff: String::new(),
            new_exp: String::new(),
            new_term: Term { coefficient: 0.0, exponent: 0.0 },
        }
    }

    pub fn handle_event(&mut self, event: char) -> Result<(), String> {
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
                _ => Err(format!("Invalid input: {}", event)),
            },
            State::ParseCaret => match event {
                '^' => {
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
                    self.new_term.coefficient = self.new_coeff.parse().map_err(|e: std::num::ParseFloatError| e.to_string())?;
                    self.new_term.exponent = self.new_exp.parse().map_err(|e: std::num::ParseFloatError| e.to_string())?;
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

        // Parse the input
        for c in input.chars() {
            dbg!("c: {}", c);
            if c == ' ' {
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