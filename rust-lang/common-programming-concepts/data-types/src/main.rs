fn main() {
    //Scaler types. Rust's scalar types are: integers, floating-point numbers, Booleans, and characters.
    let guess: u32 = "42"
    .parse() // parse means convert string to number
    .expect("Not a number!"); // expect is used to handle errors if parse fails. It returns a Result type which could be Ok or Err
    // converts string to number
    println!("The guessed number is: {guess}");

    // Integer types
    let a: i8 = -100; // 8-bit signed integer
    let b: u8 = 200; // 8-bit unsigned integer
    let c: i32 = -3000; // 16-bit signed integer
    let d: u16 = 60000; // 16-bit unsigned integer
    let e: i32 = -2000000000; // 32-bit signed integer
    let f: u32 = 4000000000; // 32-bit unsigned integer
    let g: i64 = -9000000000000000000; // 64-bit signed integer
    let h: u64 = 18000000000000000000; // 64-bit unsigned integer
    let i: i128 = -170141183460469231731687303715884105728; // 128-bit signed integer
    let j: u128 = 340282366920938463463374607431768211455; // 128-bit unsigned integer
    let k: isize = -100; // pointer-sized signed integer
    let l: usize = 100; // pointer-sized unsigned integer
    println!("Integer types: {a}, {b}, {c}, {d}, {e}, {f}, {g}, {h}, {i}, {j}, {k}, {l}");

    // Floating-point types
    let x: f32 = 3.14; // 32-bit floating-point number
    let y: f64 = 2.718281828459045; // 64-bit floating-point number
    println!("Floating-point types: {x}, {y}");

    // Boolean type
    let t: bool = true;
    let f: bool = false;
    println!("Boolean types: {t}, {f}");
    
    // Character type
    let c1: char = 'z';
    let c2: char = 'ℤ';
    let c3: char = '😊';
    println!("Character types: {c1}, {c2}, {c3}");
    
    // some operations on integers and floats
    let sum = a + 10; // addition
    let difference = b - 50; // subtraction
    let product = c * 2; // multiplication
    let quotient = d / 3; // division
    let remainder = e % 7; // modulus
    let float_sum = x + 1.0; // addition
    let float_product = y * 2.0; // multiplication
    println!("Operations: sum = {sum}, difference = {difference}, product = {product}, quotient = {quotient}, remainder = {remainder}, float_sum = {float_sum}, float_product = {float_product}");

    //Compound types: tuples and arrays
    // Tuple type
    let tup: (i32, f64, u8) = (500, 6.4, 1);
    let (x, y, z) = tup; // destructuring
    println!("Tuple: x = {x}, y = {y}, z = {z}");
    let first = tup.0; // accessing tuple elements
    let second = tup.1;
    let third = tup.2;
    println!("Tuple elements: first = {first}, second = {second}, third = {third}");

    // Array type
    let months = ["January", "February", "March", "April", "May", "June", "July", "August", "September", "October", "November", "December"];
    println!("Months: {:?}", months); // {:?} is used to print arrays
    let arr: [i32; 5] = [1, 2, 3, 4, 5]; // array of 5 integers
    let first_elem = arr[0]; // accessing array elements
    let second_elem = arr[1];
    println!("Array elements: first = {first_elem}, second = {second_elem}");
}
