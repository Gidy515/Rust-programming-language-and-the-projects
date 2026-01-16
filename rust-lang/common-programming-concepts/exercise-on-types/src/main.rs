use std::io;
fn main() {
    let a = [1, 2, 3, 4, 5];

    println!("Please enter an array index.");

    let mut index = String::new();

    io::stdin()
        .read_line(&mut index) // read_line reads user input, resulting in a Result type and appends it to the string provided
        .expect("Failed to read line"); // expect handles the Result type, causing the program to crash if there's an error

    let index: usize = index // shadowing the previous index variable
        .trim() // trim removes whitespace and newlines from the input
        .parse() // parse converts the string to a number, resulting in a Result type
        .expect("Index entered was not a number"); // expect handles the Result type, causing the program to crash if there's an error
    let element = a[index]; // this line will cause a panic if the index is out of bounds

    println!("The value of the element at index {} is: {}", index, element);


}
