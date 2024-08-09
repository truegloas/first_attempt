mod const_variable;
mod data_types;
use const_variable::*;
use data_types::*;

fn main() {
    println!("Variables and Mutability Lesson");
    var_and_mut_lesson();

    println!("\nShadowing Lesson");
    shadowing_lesson();

    println!("\nData Types Lesson");
    data_types_lesson();
}

fn var_and_mut_lesson() {
    let immutable_variable: i32 = 20;
    let mut mutable_variable: i32 = 128;

    // CONST_VARIABLE += 10;
    // immutable_variable += 10;
    mutable_variable += 10;

    let new_immut = mutable_variable * 10 - 5;
    // const NEW_CONST: i32 = new_immut - 1000 * 20 - 1000;

    println!("{} {} {}", CONST_VARIABLE, immutable_variable, mutable_variable);
    println!("Const + immut: {}", CONST_VARIABLE + immutable_variable);
    println!("Const + mut: {}", CONST_VARIABLE + mutable_variable);
    println!("Immut + mut: {}", immutable_variable + mutable_variable);
    println!("New immutable: {}", new_immut);
}

fn shadowing_example(x: i32) -> i32 {
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

fn shadowing_lesson() {
    println!("Shadowed variable: {}", shadowing_example(50));
    println!("Shadowed division: {}", shadowing_example(25));
}

fn data_types_lesson() {
    println!("Scalar Types");
    println!("Floating Point Number {PI}");
    println!("Integer Number {INTEGER_CONST_VAR}");
    println!("Unsigned Number: {UNSIGNED_INT_CONST}");

    println!("Boolean Type");
    println!("Bool Variable {F}");

    println!("Character Type");
    println!("Char Character: {LETTER_A}");
    println!("Char Emoji: {CAT}");

    let square_param: (f32, f32) = (42.5, 5.5);
    println!("Compound Type - Tuple");
    println!("Tuple: {}, {}", square_param.0, square_param.1);
    let human_param: (f32, f32, i32, &str) = human_param_to_tuple(178.0, 105.0, 24, "male");
    print!("Tuple from function: ");
    println!("{}, {}, {}, {}", human_param.0, human_param.1, human_param.2, human_param.3);
}