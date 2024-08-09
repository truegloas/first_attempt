// number after a type means amount of bits allocated in memory

pub const INTEGER_CONST_VAR: i8 = -128;
pub const UNSIGNED_INT_CONST: u8 = 255;
// There is just two types for floating point number 32 and 64 bit (f32, f64)
pub const PI: f32 = 3.1415926535;
pub const F: bool = true;
pub const LETTER_A: char = 'a';
pub const CAT: char = '😼';

pub const NAMES: [&str; 4] = ["Yura", "SerGay", "Atryom", "Mariyka"];

pub fn human_param_to_tuple(height: f32, weight: f32, age: i32, sex: &str) -> (f32, f32, i32, &str) {
    (height, weight, age, sex)
}
