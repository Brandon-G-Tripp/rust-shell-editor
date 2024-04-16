pub struct Cursor {
    line: usize,
    col: usize, 
} 

impl Cursor {
    pub fn new() -> Self {
        Self { line: 0, col: 0 }
    } 

    pub fn move_left(&mut self) {
        if self.col > 0 {
            self.col -= 1;
        } 
    } 

    pub fn move_right(&mut self) {
        self.col += 1;
    } 

    pub fn move_up(&mut self) {
        if self.line > 0 {
            self.line -= 1;
        } 
    } 

    pub fn move_down(&mut self) {
        self.line += 1;
    } 

    pub fn move_to(&mut self, line: usize, col: usize) {
        self.line = line;
        self.col = col;
    } 

    pub fn position(&self) -> (usize, usize) {
        (self.line, self.col)
    } 
} 

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new_cursor() {
        let cursor = Cursor::new();
        assert_eq!(cursor.line, 0);
        assert_eq!(cursor.col, 0);
    } 

    #[test]
    fn test_move_left() {
        let mut cursor = Cursor { line: 0, col: 5 };
        cursor.move_left();
        assert_eq!(cursor.col, 4);
    } 

    #[test]
    fn test_move_right() {
        let mut cursor = Cursor { line: 0, col: 5 };
        cursor.move_right();
        assert_eq!(cursor.col, 6);
    } 

    #[test]
    fn test_move_up() {
        let mut cursor = Cursor{ line: 5, col: 0 };
        cursor.move_up();
        assert_eq!(cursor.line, 4);
    } 

    #[test]
    fn test_move_down() {
        let mut cursor = Cursor { line: 5, col: 0 };
        cursor.move_down();
        assert_eq!(cursor.line, 6);
    } 

    #[test]
    fn test_move_to() {
        let mut cursor = Cursor { line: 0, col: 0 };
        cursor.move_to(3, 5);
        assert_eq!(cursor.line, 3);
        assert_eq!(cursor.col, 5);
    } 
} 
