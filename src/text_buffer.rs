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
        self.content.insert(pos, ch);
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
} 
