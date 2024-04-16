pub struct TextBuffer {
    content: String, 
} 

impl TextBuffer {
    pub fn new() -> Self {
        Self {
            content: String::new(),
        } 
    } 

    pub fn insert_char(&mut self, pos: usize, ch: char) {
        if ch == '\n' {
            self.content.insert_str(pos, "\n")
        } else {
            self.content.insert(pos, ch);
        }
    } 

    pub fn delete_char(&mut self, pos: usize) {
        self.content.remove(pos);
    } 

    pub fn get_line(&self, line_num: usize) -> Option<&str> {
        self.content.lines().nth(line_num)
    }

    pub fn get_lines(&self) -> Vec<&str> {
        self.content.lines().collect()
    }

    pub fn clear(&mut self) {
        self.content.clear();
    } 
} 

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new_text_buffer() {
        let buffer = TextBuffer::new();
        assert_eq!(buffer.content, "");
    } 

    #[test] 
    fn test_insert_char() {
        let mut buffer = TextBuffer::new();
        buffer.insert_char(0, 'H');
        buffer.insert_char(1, 'e');
        buffer.insert_char(2, 'l');
        buffer.insert_char(3, 'l');
        buffer.insert_char(4, 'o');
        assert_eq!(buffer.content, "Hello");
    }

    #[test]
    fn test_delete_char() {
        let mut buffer = TextBuffer::new();
        buffer.content = "Hello".to_string();
        buffer.delete_char(1);
        assert_eq!(buffer.content, "Hllo");
    } 

    #[test]
    fn test_get_line() {
        let mut buffer = TextBuffer::new();
        buffer.content = "Hello\nWorld".to_string();
        assert_eq!(buffer.get_line(0), Some("Hello"));
        assert_eq!(buffer.get_line(1), Some("World"));
        assert_eq!(buffer.get_line(2), None);
    } 

    #[test]
    fn test_get_lines() {
        let mut buffer = TextBuffer::new();
        buffer.content = "Hello\nWorld".to_string();
        assert_eq!(buffer.get_lines(), vec!["Hello", "World"]);
    } 

    #[test] 
    fn test_clear() {
        let mut buffer = TextBuffer::new();
        buffer.content = "Hello\nWorld".to_string();
        buffer.clear();
        assert_eq!(buffer.content, "");
    } 
} 
