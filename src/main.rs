fn view_string(s: &str) {
    print!("{s}");
}

fn view_number(num: i32) {
    print!("{num}");
}

fn sum(num1: i32, num2: i32) -> i32 {
    return num1 + num2;
}

fn main() {
    let s: &str = "Hi, gloas";

    let num = sum(5, -10);

    view_string(s);
    println!("");
    view_number(num);
}
