use std::io;

fn main() {
    println!("Ты гей");

    io::stdin().read_line(&mut String::new()).unwrap();
}