use std::io;

fn main() {
    let mut input = String::new();
    println!("Type Fahrenheit var : ");
    io::stdin().read_line(&mut input) 
        .expect("Failed to read string! Error!");

    let input: f32 = input.trim().parse() 
        .expect("Lmao.");

    let output: f32 = (( input - 32.0 ) * 5.0) / 9.0;
    println!("In Celsius -> {}", output);
}

