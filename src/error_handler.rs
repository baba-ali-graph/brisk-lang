use std::process;

pub struct ErrorHandler {
    had_error: bool
}

impl ErrorHandler {
    pub fn new(&self) -> ErrorHandler {
        ErrorHandler {
            had_error: false
        }
    }

    pub fn on_error(&mut self, line: u64, error_type: &String, message: &String) {
        self.had_error = true;
        eprintln!("[line {}] Error {}, {}", line, error_type, message);
        process::exit(0)
    }


}