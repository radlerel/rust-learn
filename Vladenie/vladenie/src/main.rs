fn main() {
    let my_string = String::from("hello world");
    let word = first_word(&my_string[..]);
    let my_string_literal = "hello world";
    let word = first_word(&my_string_literal[..]);
    let word = first_word(my_string_literal);
}

fn first_word(s: &str) -> &str{
    let bytes = s.as_bytes();
    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[0..i];
        }
    }
    &s[..]
}

fn takes_ownership(some_string: String) {
    println!("{}", some_string);
}

fn makes_copy(some_integer: i32) {
    println!("{}", some_integer);
}

fn gives_ownership() -> String { 
    let some_string = String::from("hello");
    some_string
}

fn takes_ownership_back(a_string: String) -> String {
    a_string
}

fn calc_len(s: &String) -> usize {
    s.len()
}

fn change(some_string: &mut String) {
    some_string.push_str(", world!");
}

