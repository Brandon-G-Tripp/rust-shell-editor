use std::io::{self, Write};

pub fn write_output<W: Write>(writer: &mut W, text: &str) -> io::Result<()> {
    writer.write_all(text.as_bytes())?;
    writer.flush()?;
    Ok(())
} 

pub fn format_output(text: &str, color: &str) -> String {
    format!("\x1b[{}m{}\x1b[0m", color, text)
} 

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_write_output() {
        // Capture the std output
        let mut output = Vec::new();

        // call the function with the test input
        let text = "Hello, world!";
        write_output(&mut output, text).unwrap();

        // Conver the captured output to a string
        let output_string = String::from_utf8(output).unwrap();

        // Assert that the output matches the expected value
        assert_eq!(output_string, "Hello, world!");
    } 

    #[test]
    fn test_format_output() {
       // Call the function with test input 
        let text = "Colored message";
        let color = "31";
        let formatted_output = format_output(text, color);

        assert_eq!(formatted_output, "\x1b[31mColored message\x1b[0m");
    } 
}
