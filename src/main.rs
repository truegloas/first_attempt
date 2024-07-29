fn var_and_mut_lesson() {
    const CONST_VARIABLE: i32 = 10;
    let immutable_variable: i32 = 20;
    let mut mutable_variable: i32 = 128;

    // CONST_VARIABLE += 10;
    // immutable_variable += 10;
    mutable_variable += 10;

    println!("{} {} {}", CONST_VARIABLE, immutable_variable, mutable_variable);
    println!("Const + immut: {}", CONST_VARIABLE + immutable_variable);
    println!("Const + mut: {}", CONST_VARIABLE + mutable_variable);
    println!("Immut + mut: {}", immutable_variable + mutable_variable);
    println!("Shadowed variable: {}", shadowing_lesson(50));
    println!("Shadowed division: {}", shadowing_lesson(25));
}

fn main() {
    var_and_mut_lesson();
}

fn shadowing_lesson(x: i32) -> i32 {
    {
        let x = x * 2;
        if x == 50 {
            return x / 2;
        } else {
            println!("{}", x * 2);
        }
    }

    x
}