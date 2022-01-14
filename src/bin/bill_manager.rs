use std::io;

fn main() {}

fn get_input() -> Option<String> {
    let mut buff = String::new();
    while io::stdin().read_line(&mut buff).is_err() {
        println!("Please enter your data again.");
    }
    let input = buff.trim().to_owned();
    if input == "" {
        return None;
    } else {
        return Some(input);
    }
}
