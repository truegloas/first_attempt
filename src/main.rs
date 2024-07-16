fn main() {
    let mut summer: i32 = 0;

    loop {
        println!("Enter numbers to sum: ");

        let mut number = String::new();
        std::io::stdin()
            .read_line(&mut number)
            .expect("Failed to read the line");

        let number: i32 = match number.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("Not a number");
                continue;
            }
        };

        summer += number;
        println!("Summered number: {summer}");
    }
}
