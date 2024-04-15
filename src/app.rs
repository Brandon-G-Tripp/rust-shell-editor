use crate::io::{input, output};

pub struct App {
    running: bool,
}

impl App {
    pub fn new() -> Self {
        App {
            running: true,
        }
    }

    pub fn run<R: std::io::BufRead, W: std::io::Write>(&mut self, reader: &mut R, writer: &mut W) {
        if std::env::var("RUST_TEST_ENV").is_ok() {
            // exit gracefully for testing purposes
            return;
        } 

        output::write_output(writer, "Welcome to the Upepo Text Editor with Integrated Shell!\n").unwrap();

        while self.running {
            output::write_output(writer, "> ").unwrap();
            let command = input::read_line(reader).unwrap();

            if command == "exit" {
                self.running = false;
                output::write_output(writer, "Exiting...\n").unwrap();
                break;
            } else {
                // Execute the command and display the output 
                output::write_output(writer, &format!("Output of {}\n", command)).unwrap();
            } 

        } 
    } 
} 

#[cfg(test)]
mod tests {
    use super::*;
    use std::io::Cursor;

    #[test]
    fn test_app_creation() {
        let app = App::new();
        assert!(app.running);
    } 

    #[test]
    fn test_app_run_exit() {
        let mut input = Cursor::new(b"exit\n");
        let mut output = Vec::new();

        let mut app = App::new();
        app.run(&mut input, &mut output);

        assert!(!app.running);
        let expected_output = "Welcome to the Upepo Text Editor with Integrated Shell!\n> Exiting...\n";
        assert_eq!(String::from_utf8(output).unwrap(), expected_output);
        
    } 

    #[test]
    fn test_app_run() {
        let mut input = Cursor::new(b"command1\ncommand2\nexit\n");
        let mut output = Vec::new();

        let mut app = App::new();
        app.run(&mut input, &mut output);

        let expected_output = "Welcome to the Upepo Text Editor with Integrated Shell!\n> Output of command1\n> Output of command2\n> Exiting...\n";
        assert_eq!(String::from_utf8(output).unwrap(), expected_output);
    } 
}
