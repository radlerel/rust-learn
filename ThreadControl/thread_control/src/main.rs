fn main() {
    let number: i32 = 6;
    if number % 4 == 0 {
        println!("Number can on 4 : ");
    } else if number % 3 == 0 {
        println!("Number can on 3 : ");
    } else if number % 2 == 0 {
        println!("Number can on 2 : ");
    } else {
        println!("Number can't on 4, 3 or 2");
    }
    condit();
    cycle();
    cycle_second();
    array();
    for_example();
}
fn condit() {
    let condition = false;
    let number = if condition {
        5
    } else {
        6
    };

    println!("Number var is - {}", number);
}

fn cycle() {
    let mut counter: i32 = 0;
    let result = loop {
        counter += 1;

        if counter == 10 {
            break counter * 2;
        }
    };
    println!("Result is : {}", result);
}

fn cycle_second() {
    let mut number: i32 = 13;
    while number != 0 {
        println!("{}!", number);
        number = number - 1;
    }
    println!("Let's go!");
}

fn array() {
    let a = [10, 20, 30, 40, 50];
    let mut index = 0;
    while index < 5 {
        println!("Current number is {}", a[index]);

        index = index + 1;
    }
}

fn for_example() {
    for number in (1..4).rev() {
        println!("{}", number);
    }
    println!("LET'S GO!!!");
}