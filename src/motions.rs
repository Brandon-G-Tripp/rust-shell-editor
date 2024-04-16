// use crate::terminal::Terminal;
// use std::io::{self, Write};

// pub enum Motion {
//     Left, 
//     Right, 
//     Up, 
//     Down,
// } 

// pub fn handle_motion(motion: Motion, terminal: &mut Terminal) {
//     match motion {
//         Motion::Left => move_left(terminal),
//         Motion::Right => move_right(terminal),
//         Motion::Up => move_up(terminal),
//         Motion::Down => move_down(terminal),
//     } 
// } 

// fn move_left(terminal: &mut Terminal) -> io::Result<()> {
//     let (mut x, y) = terminal.get_cursor_position();
//     if x > 0 {
//         x -= 1;
//         terminal.set_cursor_position(&mut terminal.writer, x, y)?;
//     } 
//     Ok(())
// } 

// fn move_right(terminal: &mut Terminal) -> io::Result<()> {
//     let (mut x, y) = terminal.get_cursor_position();
//     x += 1;
//     terminal.set_cursor_position(&mut terminal.writer, x, y)?;
//     Ok(())
// } 

// fn move_up(terminal: &mut Terminal) -> io::Result<()> {
//     let (x, mut y) = terminal.get_cursor_position();
//     if y > 0 {
//         y -= 1;
//         terminal.set_cursor_position(&mut terminal.writer, x, y)?;
//     } 
//     Ok(())
// } 

// fn move_down(terminal: &mut Terminal) -> io::Result<()> {
//     let (x, mut y) = terminal.get_cursor_position();
//     y += 1;
//     terminal.set_cursor_position(&mut terminal.writer, x, y)?;
//     Ok(())
// } 

// #[cfg(test)]
// mod tests {
//     use super::*;
//     use crate::terminal::Terminal;
//     use std::io::Cursor;

//     #[test]
//     fn test_handle_motion() {
//         let mut mock_output = Cursor::new(Vec::new());
//         let mut terminal = Terminal::new(&mut mock_output);

//         // Set initial cursor position
//         terminal.set_cursor_position(5, 5).unwrap();

//         // Test moving left
//         handle_motion(&mut terminal, Motion::Left).unwrap();
//         assert_eq!(terminal.get_cursor_position(), (4, 5));

//         // Test moving right
//         handle_motion(&mut terminal, Motion::Right).unwrap();
//         assert_eq!(terminal.get_cursor_position(), (5, 5));

//         // Test moving up
//         handle_motion(&mut terminal, Motion::Up).unwrap();
//         assert_eq!(terminal.get_cursor_position(), (5, 4));

//         // Test moving down
//         handle_motion(&mut terminal, Motion::Down).unwrap();
//         assert_eq!(terminal.get_cursor_position(), (5, 5));
//     }
// }
