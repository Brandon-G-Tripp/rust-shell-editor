pub struct Renderer {
    scroll: usize,
}

impl Renderer {
    pub fn new() -> Self {
        Self { scroll: 0 }
    } 
    pub fn clear_screen<W: Write>(&self, writer: &mut W) -> io::Result<()> {
        write!(writer, "{}{}", clear::All, color::Fg(color::Reset))?;
        Ok(())
    } 

    pub fn draw_text<W: Write>(&self, text: &str, x: usize, y: usize) {
        let terminal_width = termion::terminal_size().unwrap().0 as usize;
        let mut line_width = 0;
        for (i, c) in text.chars().enumerate() {
            if line_width >= terminal_width {
                write!(writer, "\n")?;
                line_width = 0;
            } 
            write!(writer, "{}", c)?;
            line_width += 1;
        } 
        Ok(())
    } 

    pub fn draw_cursor<W: Write>(&self, x: usize, y: usize) {
        write!(writer, "{}", termion::cursor::Goto(x as u16 + 1, y as u16 + 1));
    }

    pub fn render<W: Write>(&self, text_buffer: &TextBuffer, cursor: &Cursor) {
        self.clear_screen(writer)?;
        let lines = text_buffer.get_lines();
        let terminal_height = termion::terminal_size().unwrap().1 as usize;
        for (y, line) in lines.iter().skip(self.scroll).take(terminal_height).enumerate() {
           self.draw_text(writer, line, 0, y)?; 
        } 
        let (cursor_x, cursor_y) = cursor.position();
        self.draw_cursor(writer, cursor_x, cursor_y)?;
        writer.flush()?;
        Ok(())
    } 

    pub fn scroll_up(&mut self) {
        self.scroll = self.scroll.saturating_sub(1);
    } 

    pub fn scroll_down(&mut self) {
        self.scroll = self.scroll.saturating_add(1);
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

    #[test]
   fn test_render_with_scrolling() {
       let mut renderer = Renderer::new();
       let mut output = Vec::new();
       let mut text_buffer = TextBuffer::new();
       text_buffer.insert_line(0, "Line 1".to_string());
       text_buffer.insert_line(1, "Line 2".to_string());
       text_buffer.insert_line(2, "Line 3".to_string());
       let cursor = Cursor::new();
       cursor.move_to(1, 0);

       // Render without scrolling
       renderer.render(&mut output, &text_buffer, &cursor).unwrap();
       let expected_output = format!(
           "{}{}Line 1\nLine 2\nLine 3\n{}",
           clear::All,
           color::Fg(color::Reset),
           cursor::Goto(1, 2)
       );
       assert_eq!(String::from_utf8(output.clone()).unwrap(), expected_output);

       // Render with scrolling
       renderer.scroll_down();
       output.clear();
       renderer.render(&mut output, &text_buffer, &cursor).unwrap();
       let expected_output = format!(
           "{}{}Line 2\nLine 3\n{}",
           clear::All,
           color::Fg(color::Reset),
           cursor::Goto(1, 1)
       );
       assert_eq!(String::from_utf8(output).unwrap(), expected_output);
   }

    #[test]
   fn test_render_with_line_wrapping() {
       let renderer = Renderer::new();
       let mut output = Vec::new();
       let mut text_buffer = TextBuffer::new();
       text_buffer.insert_line(0, "This is a long line that should wrap".to_string());
       let cursor = Cursor::new();
       cursor.move_to(0, 0);

       renderer.render(&mut output, &text_buffer, &cursor).unwrap();
       let expected_output = format!(
           "{}{}This is a long line that \nshould wrap\n{}",
           clear::All,
           color::Fg(color::Reset),
           cursor::Goto(1, 1)
       );
       assert_eq!(String::from_utf8(output).unwrap(), expected_output);
   }
} 
