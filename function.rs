fn main() {
    greet("Alice");
    let result = add(3, 5);
    println!("Result: {}", result);
}

fn greet(name: &str) {
    println!("Hello, {}!", name);
}

fn add(a: i32, b: i32) -> i32 {
    a + b // Implicit return
}
