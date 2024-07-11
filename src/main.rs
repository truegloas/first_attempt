fn view_string(s: &str) {
    print!("{s}");
}

fn view_number(num: i32) {
    print!("{num}");
}

fn sum(num1: i32, num2: i32) -> i32 {
    return num1 + num2;
}

fn view_array(num_array:[i32; 8]) {
    for i in num_array {
        print!("{i} ");
    }
}

fn enter_num() -> i32 {
    let mut input_string = String::new();
    std::io::stdin().read_line(&mut input_string).unwrap();
    return input_string.trim().parse().unwrap();
}

fn main() {
    let s: &str = "Hi, gloas";

    let num = sum(5, -10);

    view_string(s);
    println!("");
    view_number(num);

    println!("");

    let mut numbers = [134, 235645, 65643, 643214, 53412, 6234, 7234, 81342];
    println!("{:?}", numbers); // [1, 2, 3, 4, 5, 6, 7, 8],
    println!("ты гей {}", numbers[3]);
    numbers[3] = 33122i32;

    for i in 1..10 {
        view_number(i);
        print!(" ");
    }

    println!();
    view_array(numbers);
    println!();

    println!("Input number: ");
    let input_num = enter_num();
    println!("{}", input_num);
}
