fn main() {
    println!("Hello, world!");
    print_labeled_measurement(5, 'h');

    // Statements and Expressions
    let y = {
        let x = 3;
        x + 1 // No semicolon, so this is an expression that returns a value
    };
    println!("The value of y is: {}", y);

    let z = five();
    println!("The value of z is: {}", z);

    let a = plus_one(5);
    println!("The value of a is: {}", a);
}

fn print_labeled_measurement (value: i32, unit_label: char) {
    println!("The measurement is: {} {}", value, unit_label);
}

// Functions with Return Values
fn five() -> i32 {
    5 // No semicolon, so this is an expression that returns a value
}

fn plus_one(x: i32) -> i32 {
  return x + 1; // No semicolon, so this is an expression that returns a value
}

// Comments
// This is a single-line comment
/* This is a
   multi-line comment */



