use std::io;

fn main() -> io::Result<()> {
    println!("Enter your name: ");

    let mut buffer = String::new();
    io::stdin().read_line(&mut buffer)?;

    println!("Hello, {}", buffer);

    Ok(())
}
