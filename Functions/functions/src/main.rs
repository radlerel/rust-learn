fn main() {
    another_function(4, 5);
    rail();
    let x = return_fine();
    another_function_two(x);
    let y: i32 = plus_one(x);
    another_function_two(y);
}
fn another_function_two(x: i32) {
    println!("Your number is : {}", x);
}
fn another_function(x: i32, y: i32) {
    println!("Your X is : {}", x);
    println!("Your Y is : {}", y);
}
fn rail() {
    let _x: i32 = 5;
    let y = {
        let x= 3;
        x + 1
    };
    println!("Y is : {}", y)
}

fn return_fine() -> i32 {
    5
}

fn plus_one(x: i32) -> i32 {
    x + 1
}

