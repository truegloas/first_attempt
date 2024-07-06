fn view_string(s: &str) {
    print!("{s}");
}

fn view_number(num: i32) {
    print!("{num}");
}

fn main() {
    let s: &str = "Hi, gloas";

    let num = 8;

    view_string(s);
    println!("");
    view_number(num);
}
