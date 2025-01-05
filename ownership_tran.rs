fn main() {
    let s = String::from("hello"); // Declare a string
    let len = calculate_length(&s); // Borrow the string
    println!("Length: {}", len); // Print the length
}

fn calculate_length(s: &String) -> usize {
    s.len() // Return the length of the string
}
