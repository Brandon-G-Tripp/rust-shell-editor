use std::io::{self, Write};

use upepo::io::{input, output};

fn main() {
    // Read cmd line args
    let args = input::read_args();
    println!("Command line arguments: {:?}", args);

    // Prompt the user for input
    print!("Enter a command: ");
    io::stdout().flush().unwrap();
    match input::read_line(&mut io::stdin().lock()) {
        Ok(command) => {
            println!("Entered command: {}", command);
        }
        Err(err) => {
            eprintln!("Error reading input: {}", err);
        }
    } 

    // Write output to stdout
    let message = "Hello, world!";
    match output::write_output(&mut io::stdout(), message) {
        Ok(_) => println!("Output written successfully"),
        Err(err) => eprintln!("Error writing output: {}", err),
    } 

    // Format output with color
    let colored_message = output::format_output("Colored message", "31");
    println!("{}", colored_message);
}
