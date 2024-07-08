use std::io;

fn main() {
    println!("Enter your name: ");

    let mut buffer = String::new();
    io::stdin().read_line(&mut buffer).unwrap();

    print!("Hello, {}", buffer);

    buffer = "".to_string();
    io::stdin().read_line(&mut buffer).unwrap();
    println!("{}", buffer.trim().parse::<i32>().expect("Not a number"));
}
