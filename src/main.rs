use std::io;

fn main() {
    println!("Enter your name: ");

    let mut buffer = String::new();
    io::stdin().read_line(&mut buffer).unwrap();

    println!("Hello, {}", buffer);
}
