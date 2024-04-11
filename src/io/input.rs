use std::env;
use std::io::{self, BufRead, BufReader, Read};

pub fn read_args() -> Vec<String> {
    env::args().collect()
} 

pub fn read_line<R: BufRead>(reader: &mut R) -> Result<String, io::Error> {
    let mut input = String::new();
    reader.read_line(&mut input)?;
    Ok(input.trim().to_string())
} 

pub fn read_from_file<R: Read>(reader: R) -> io::Result<String> {
    let mut reader = BufReader::new(reader);
    let mut content = String::new();
    reader.read_to_string(&mut content)?;
    Ok(content)
} 


#[cfg(test)]
mod tests {
    use super::*;
    use std::process::Command;
    use std::fs::File;
    use std::io::Write;

    #[test]
    fn test_read_args() {
        // Spawn a new process with the desired command line arguments
        let output = Command::new("cargo")
            .arg("run")
            .arg("--")
            .arg("arg1")
            .arg("arg2")
            .arg("arg3")
            .output()
            .expect("Failed to execute process");

        // Convert the output to a string
        let output_string = String::from_utf8_lossy(&output.stdout);

        // Split the output string into lines
        let lines: Vec<&str> = output_string.split('\n').collect();

        // Find the line that contains the command line arguments
        let args_line = lines
            .iter()
            .find(|line| line.starts_with("Command line arguments: "))
            .expect("Command line arguments not found in output");

        // Extract the command line arguments from the line
        let args_str = args_line.trim_start_matches("Command line arguments: ");
        let args: Vec<&str> = args_str[1..args_str.len() - 1].split(", ").collect();


        // Remove executable path and surrounding quotation marks
        let cleaned_args: Vec<&str> = args
            .iter()
            .skip(1) // skip first arg (exe path)
            .map(|arg| arg.trim_matches('"'))
            .collect();

        // Assert that the extracted arguments match the expected values
        assert_eq!(cleaned_args, vec!["arg1", "arg2", "arg3"]);
    } 


    #[test]
    fn test_read_line() {
        // Simulate user input 
        let input = "Hello, world!\n";
        let mut reader = io::Cursor::new(input.as_bytes());

        // Calll the function and check the reuslt
        let result = read_line(&mut reader);
        assert_eq!(result.unwrap(), "Hello, world!");
    } 

    #[test]
    fn test_read_from_file() {
        // Create a temporary file with sample content
        let mut file = File::create("test_input.txt").unwrap();
        file.write_all(b"Hello, world!").unwrap();

        // Open the file for reading
        let file = File::open("test_input.txt").unwrap();

        // Call the function to read from the file
        let result = read_from_file(file);

        // Assert that the content is read correctly
        assert_eq!(result.unwrap(), "Hello, world!");
    }
}
