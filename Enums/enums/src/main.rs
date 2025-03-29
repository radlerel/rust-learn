
#[derive(Debug)]

enum UsState {
    Alabama,
    Alaska,
}
enum Message {
    Quit,
    Move { x: i32, y: i32},
    Write(String),
    ChangeColor(i32, i32, i32),
}

enum Coin {
    Penny,
    Nickel,
    Dime,
    Quater(UsState),
}
enum Option<T> {
    Some(T),
    None,
}

impl Message {
    fn call(&self) {
    }
}
enum IpAddrKind {
    V4,
    V6,
}

struct IpAddr {
    kind: IpAddrKind,
    adress: String,
}

fn plus_one(x: Option<i32>) -> Option<i32> {
    match x {
        Option::None => Option::None,
        Option::Some(i) => Option::Some(i + 1),
    }
}

fn main() {
    let four = IpAddrKind::V4;
    let six = IpAddrKind::V6;
    route(IpAddrKind::V4);
    route(IpAddrKind::V6);
    let home = IpAddr {
        kind: IpAddrKind::V4,
        adress: String::from("127.0.0.1"),
    };
    
    let loopback = IpAddr {
        kind: IpAddrKind::V6,
        adress: String::from("::1"),
    };

    let m = Message::Write(String::from("hello"));
    m.call();

    let some_number = Some(5);
    let some_string = Some("Hi!");
    let absent_num: Option<i32> = Option::None;

    let five = Option::Some(5);
    let six = plus_one(five);
    let none = plus_one(Option::None);

    let some_u8_var = Option::Some(0u8);
    match some_u8_var {
        Option::Some(1) => println!("one"),
        Option::Some(3) => println!("three"),
        Option::Some(4) => println!("five"),
        Option::Some(7) => println!("seven"),
        _ => (),
    }

    value_in_cents(Coin::Penny);

    if let Option::Some(3) = some_u8_var {
        println!("three");
    }

    let mut count = 0;
    let coin = Coin::Quater(UsState::Alaska);
    if let Coin::Quater(state) = coin {
        println!("Quater from state {:?}", state);
    } else {
        count += 1;
    }
}

fn route(ip_kind: IpAddrKind) {
}

fn value_in_cents(coin: Coin) -> u8 {
    match coin {
        Coin::Penny => {
            println!("LUCK!");
            1
        },
        Coin::Nickel => 5,
        Coin::Dime => 10,
        Coin::Quater(state) => {
            println!("Quater from {:?} state!", state);
            25
        },
    }
}
