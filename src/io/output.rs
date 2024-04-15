use std::io::{self, Write};
use termion::{clear, cursor};

pub fn write_text<W: Write>(writer: &mut W, text: &str) -> io::Result<()> {
    writer.write_all(text.as_bytes())?;
    writer.flush()
}

pub fn clear_screen<W: Write>(writer: &mut W) -> io::Result<()> {
    write!(writer, "{}", clear::All)?;
    writer.flush()
}

pub fn set_cursor_position<W: Write>(writer: &mut W, x: u16, y: u16) -> io::Result<()> {
    write!(writer, "{}", cursor::Goto(x, y))?;
    writer.flush()
}

pub fn write_output<W: Write>(writer: &mut W, text: &str) -> io::Result<()> {
    writer.write_all(text.as_bytes())?;
    writer.flush()?;
    Ok(())
} 

pub fn format_output(text: &str, color: &str) -> String {
    format!("\x1b[{}m{}\x1b[0m", color, text)
} 

pub fn write_to_file<W: Write>(mut writer: W, text: &str) -> io::Result<()> {
    writer.write_all(text.as_bytes())?;
    writer.flush()?;
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::{fs::File, io::Read};

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

     #[test]
    fn test_write_to_file() {
        // Create a temporary file
        let file = File::create("test_output.txt").unwrap();

        // Call the function to write to the file
        let text = "Hello, file!";
        write_to_file(file, text).unwrap();

        // Open the file for reading
        let mut file = File::open("test_output.txt").unwrap();
        let mut content = String::new();
        file.read_to_string(&mut content).unwrap();

        // Assert that the content is written correctly
        assert_eq!(content, "Hello, file!");
    }
}
