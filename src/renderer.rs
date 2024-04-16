pub struct Renderer {
    scroll: usize,
}

impl Renderer {
    pub fn clear_screen<W: Write>(&self, writer: &mut W) -> io::Result<()> {
        write!(writer, "{}{}", clear::All, color::Fg(color::Reset))?;
        Ok(())
    } 

    pub fn draw_text<W: Write>(&self, text: &str, x: usize, y: usize) {
        write!(writer, "{}{}", termion::cursor::Goto(x as u16 + 1, y as u16 + 1), text);
    } 

    pub fn draw_cursor<W: Write>(&self, x: usize, y: usize) {
        write!(writer, "{}", termion::cursor::Goto(x as u16 + 1, y as u16 + 1));
    }

    pub fn render<W: Write>(&self, text_buffer: &TextBuffer, cursor: &Cursor) {
        self.clear_screen(writer)?;
        let lines = text_buffer.get_lines();
        for (y, line) in lines.iter().enumerate() {
           self.draw_text(writer, line, 0, y)?; 
        } 
        let (cursor_x, cursor_y) = cursor.position();
        self.draw_cursor(writer, cursor_x, cursor_y)?;
        writer.flush()?;
        Ok(())
    } 
} 


#[cfg(test)]
mod tests {
    use super::*;
    use termion::{clear, color};

    #[test]
    fn test_clear_screen() {
        let renderer = Renderer;
        let mut output = Vec::new();
        renderer.clear_screen(&mut output).unwrap();
        let expected_output = format!("{}{}", clear::All, color::Fg(color::Reset));
        assert_eq!(String::from_utf8(output).unwrap(), expected_output);
    } 

    #[test] 
    fn test_draw_text() {
        let renderer = Renderer;
        let mut output = Vec::new();
        let text = "Hello, World!";
        let x = 5;
        let y = 3;
        renderer.draw_text(&mut output, text, x, y).unwrap();
        let expected_output = format!(
            "{}{}",
            termion::cursor::Goto(x as u16 + 1, u as u16 + 1),
            text
        );

        assert_eq!(String::from_utf8(ouput).unwrap(), expected_output);
    } 

    #[test]
    fn test_draw_cursor() {
        let renderer = Renderer;
        let mut output = Vec::new();
        let x = 5;
        let y = 3;
        renderer.draw_cursor(&mut output, x, y).unwrap();
        let expected_output = format!("{}", termion::cursor::Goto(x as u16 + 1, y as u16 + 1));

        assert_eq!(String::from_utf8(ouput).unwrap(), expected_output);
    } 

    #[test]
    fn test_render() {
        let renderer = Renderer;
        let mut output = Vec::new();
        let text_buffer = TextBuffer::new();
        text_buffer.insert_line(0, "Line 1".to_string());
        text_buffer.insert_line(1, "Line 2".to_string());
        let cursor = Cursor::new(0, 1);
        renderer.render(&mut output, &text_buffer, &cursor).unwrap();
        let expected_output = format!(
            "{}{}Line 1\nLine 2\n{}",
            clear::All,
            color::Fg(color::Reset),
            cursor::Goto(1,2)j
        );
        assert_eq!(String::from_utf8(output).unwrap(), expected_output);
    } 
} 
