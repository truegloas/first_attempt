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
}

fn main() {
    var_and_mut_lesson();
}