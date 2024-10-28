use std::io;

fn game() {
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    let input = input.trim();
    println!("{}", input);
}

fn main() {
    game();
}
