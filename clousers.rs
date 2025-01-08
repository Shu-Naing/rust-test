fn main() {
    let add = |x, y| x + y;
    let result = add(5, 6);
    println!("The result of addition is: {}", result);

    let closure_with_capture = |x| {
        let y = 10;
        x + y
    };

    let result_captured = closure_with_capture(5);
    println!("Captured closure result: {}", result_captured);
}
