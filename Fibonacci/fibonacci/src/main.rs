use std::io; 
fn main() {
    let mut n = String::new();
    println!("Wirte n~ number here : ");
    io::stdin().read_line(&mut n)
        .expect("Cant't read line!");
    let number: i32 = n.trim().parse() 
        .expect("Lmao.");
    let mut current_number: i32 = 0;
    let mut next_number: i32 = 1;
    let mut cycles: i32 = 0;
    while cycles < number {
        next_number = current_number + next_number;
        current_number = next_number - current_number;
        cycles += 1;
        println!("Fibonacci is => {}", current_number);
    }
}
