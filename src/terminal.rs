use std::io::{self, Write, Read, BufRead};

use termion::event::Key;

use crate::io::input::{ read_key, read_line};
use crate::io::output::{write_text, clear_screen, set_cursor_position};

pub struct Terminal {
    size: (u16, u16),
} 

impl Terminal {
    pub fn new() -> Self {
        let size = termion::terminal_size().expect("Failed to get terminal size");
        Terminal {
            size,
        }
    } 

    pub fn read_key<R: Read>(&mut self, reader: &mut R) -> Result<Key, io::Error> {
        match read_key(reader) {
            Ok(key) => Ok(convert_key(key)),
            Err(err) => Err(err),
        } 
    } 

    pub fn read_line<R: BufRead>(&mut self, reader: &mut R) -> Result<String, io::Error> {
        read_line(reader)
    } 

    pub fn write_text<W: Write>(&mut self, writer: &mut W, text: &str) -> io::Result<()> {
        write_text(writer, text)
    } 

    pub fn clear_screen<W: Write>(&mut self, writer: &mut W) -> io::Result<()> {
        clear_screen(writer)
    } 

    pub fn set_cursor_position<W: Write>(&mut self, writer: &mut W, x: u16, y: u16) -> io::Result<()> {
        set_cursor_position(writer, x, y)
    } 


}

fn convert_key(key: char) -> Key {
    Key::Char(key)
} 

#[cfg(test)]
mod tests {
    use super::*;
    use std::io::Cursor;

    #[test]
    fn test_read_key() {
        let input = b"a\n";
        let mut reader = Cursor::new(input);
        let mut terminal = Terminal::new();
        match terminal.read_key(&mut reader){
            Ok(key) => assert_eq!(key, Key::Char('a')),
            Err(err) => {
                eprintln!("Error reading key: {}", err);
                assert!(false, "Failed to read key");
            } 

        }
    }

    #[test]
    fn test_write_text() {
        let mut mock_output = Vec::new();
        let mut terminal = Terminal::new();
        terminal.write_text(&mut mock_output, "Hello, world!").unwrap();
        assert_eq!(String::from_utf8(mock_output).unwrap(), "Hello, world!");
    }
} 
