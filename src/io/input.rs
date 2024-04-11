use std::env;
use std::io::{self, BufRead};

pub fn read_args() -> Vec<String> {
    env::args().collect()
} 

pub fn read_line() -> Result<String, io::Error> {
    let mut input = String::new();
    io::stdin().lock().read_line(&mut input)?;
    Ok(input.trim().to_string())
} 
