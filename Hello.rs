use std::io;
fn main() {
    println!("Hello World!");
    println!("I'm a Rustacean!");
    let mut name = String::new();
    println!("Enter your name:");
    io::stdin().read_line(&mut name).expect("Failed to read line");
    println!("Hello, {}!", name.trim());
}
