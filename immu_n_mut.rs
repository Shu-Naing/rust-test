fn main() {
    let mut counter = 0; // Mutable variable for counting
    for i in 1..5 {
        counter += i;
        println!("Counter: {}", counter);
    }

    let name = "Rust"; // Immutable variable
    println!("Programming Language: {}", name);
}
