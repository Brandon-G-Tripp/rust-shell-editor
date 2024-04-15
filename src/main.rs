use std::io::{self};

use upepo::app::App;
use upepo::io::input;

fn main() {
    let args = input::read_args();
    println!("Command line arguments: {:?}", args);

    let mut app = App::new();

    app.run(&mut io::stdin().lock(), &mut io::stdout());
}
