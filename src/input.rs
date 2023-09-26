use std::io::{self, Write};

pub fn get_input_string(prompt: &str) -> String {
    let mut input = String::new();
    println!("\n{}", prompt);
    io::stdin().read_line(&mut input).unwrap();
    input.trim().to_string()
}
