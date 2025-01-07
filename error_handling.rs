fn divide(a: f64, b: f64) -> Option<f64> {
    if b == 0.0 {
        None
    } else {
        Some(a / b)
    }
}

fn main() {
    let a = 10.0;
    let b = 2.0;

    match divide(a, b) {
        Some(result) => println!("Result: {}", result),
        None => println!("Cannot divide by zero!"),
    }
}
