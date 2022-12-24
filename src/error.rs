

#[derive(Debug, Clone)]
pub struct InterpreterError {
    pub line: usize,
    pub location: String,
    pub message: String,
    local: bool
}

impl InterpreterError {
    pub fn new_local(line: usize, location: &str, message: &str) -> Self {
        InterpreterError {
            line,
            location: location.to_owned(),
            message: message.to_owned(),
            local: true
        }
    }
    
    pub fn new(message: &str) -> Self {
        InterpreterError {
            line: 0,
            location: String::new(),
            message: message.to_owned(),
            local: false
        }
    }
    
    pub fn report(&self) {
        if self.local {eprintln!("Interpreter Error: [line {}] Error {}: {}", self.line, self.location, self.message);}
        else {eprintln!("Interpreter Error: {}", self.message);}
    }
}
