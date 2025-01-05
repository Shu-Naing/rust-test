const INTEGER: i32 = 10; // Use const for global constants
const FLOATING_POINT: f64 = 3.14;
const BOOLEAN: bool = true;
const CHARACTER: char = 'R';

fn main() {
    println!("Hello, Rust!");
    println!("Integer: {}", INTEGER); // Correctly reference the variable
    println!("Floating Point: {}", FLOATING_POINT);
    println!("Boolean: {}", BOOLEAN);
    println!("Character: {}", CHARACTER);
}
