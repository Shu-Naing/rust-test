fn main() {
    // Example 1: Matching Numbers
    let number = 42;
    match number {
        1 => println!("The number is one."),
        2 | 3 | 5 | 7 => println!("The number is a prime number less than 10."),
        10..=50 => println!("The number is between 10 and 50."),
        _ => println!("The number is something else."),
    }

    // Example 2: Matching Option
    let maybe_value: Option<i32> = Some(100);
    match maybe_value {
        Some(x) => println!("We have a value: {}", x),
        None => println!("No value present."),
    }

    // Example 3: Matching Tuples
    let pair = (3, 5);
    match pair {
        (x, y) if x == y => println!("The numbers are equal."),
        (x, y) if x + y == 10 => println!("The sum of the pair is 10."),
        (_, _) => println!("Some other pair."),
    }

    // Example 4: Matching Enums
    enum Color {
        Red,
        Green,
        Blue,
        Rgb(u8, u8, u8),
    }

    let color = Color::Rgb(255, 0, 0);
    match color {
        Color::Red => println!("The color is red."),
        Color::Green => println!("The color is green."),
        Color::Blue => println!("The color is blue."),
        Color::Rgb(r, g, b) => println!("The color is an RGB value: ({}, {}, {}).", r, g, b),
    }

    // Example 5: Matching Arrays
    let arr = [1, 2, 3];
    match arr {
        [1, x, y] => println!("Starts with 1, other values are: {} and {}", x, y),
        [a, b, c] if a + b + c == 6 => println!("All values add up to 6."),
        _ => println!("Something else."),
    }

    // Example 6: Ignoring Values
    let some_tuple = (5, 10, 15);
    match some_tuple {
        (x, _, z) => println!("First and last values are: {} and {}", x, z),
    }

    // Example 7: Combining Patterns
    let age = 18;
    match age {
        0..=12 => println!("Child"),
        13..=19 => println!("Teenager"),
        20..=59 => println!("Adult"),
        _ => println!("Senior"),
    }

    // Example 8: Matching References
    let reference = &4;
    match reference {
        &val => println!("Got a reference to value: {}", val),
    }

    // Example 9: Destructuring Structs
    struct Point {
        x: i32,
        y: i32,
    }

    let point = Point { x: 10, y: 20 };
    match point {
        Point { x, y } => println!("Point coordinates: ({}, {})", x, y),
    }

    // Example 10: Using @ Bindings
    let value = Some(10);
    match value {
        Some(x @ 1..=10) => println!("Matched a value in range 1 to 10: {}", x),
        Some(_) => println!("Matched some value."),
        None => println!("Matched no value."),
    }
}
