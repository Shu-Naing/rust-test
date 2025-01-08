fn main() {
    // Example 1: Using Option
    let some_value = Some(42);
    let no_value: Option<i32> = None;

    match some_value {
        Some(v) => println!("Found: {}", v),
        None => println!("No value found."),
    }

    match no_value {
        Some(v) => println!("Found: {}", v),
        None => println!("No value found."),
    }

    // Example 2: Using Result
    let result = divide(10, 2);
    match result {
        Ok(v) => println!("Division result: {}", v),
        Err(e) => println!("Error: {}", e),
    }

    let result_err = divide(10, 0);
    match result_err {
        Ok(v) => println!("Division result: {}", v),
        Err(e) => println!("Error: {}", e),
    }
}

fn divide(a: i32, b: i32) -> Result<i32, String> {
    if b == 0 {
        Err(String::from("Cannot divide by zero"))
    } else {
        Ok(a / b)
    }
}
