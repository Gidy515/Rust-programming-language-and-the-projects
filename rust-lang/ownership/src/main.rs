fn main() {
    // Variable scope
    { // start of a new scope, s is not valid here since it is not yet declared
        let s = "hello"; // s is valid from this point forward (A string literal)
        println!("{}", s); // prints "hello"
    } // s is no longer valid here, as it has gone out of scope

    // The String type
    let mut t = String::from("hello"); // t is a String, allocated on the heap
    t.push_str(", world!"); // push_str() appends a literal to a String
    println!("{t}"); // prints "hello, world!"

    // Variables and Data Interacting with Move
    let x = 5; // x is an integer, which is a Copy type
    let y = x; // y is a copy of x, both are valid. this is because integers are stored on the stack which can be copied
    println!("x = {}, y = {}", x, y); // prints "x = 5, y = 5"

    let s1 = String::from("hello"); // s1 is a String, which is not a Copy type
    let s2 = s1; // s2 takes ownership of the String, s1 is no longer valid
    // println!("{}, world!", s1); // this would cause a compile-time error because s1 is no longer valid
    println!("{s2}, world!"); // prints "hello, world!"

    // Variables and Data Interacting with Clone
    let s3 = String::from("hello"); // s3 is a String
    let s4 = s3.clone(); // s4 is a clone of s3, both are valid
    println!("s3 = {s3}, s4 = {s4}"); // prints "s3 = hello, s4 = hello"

    // Ownership and Functions
    let s5 = String::from("heyo"); // s5 is a String
    takes_ownership(s5); // s5's value moves into the function, and is no longer valid here
    // println!("{s5}"); // this would cause a compile-time error because s5 is no longer valid

    let r = 5; // r comes into scope
    makes_copy(r); // r is copied into the function because i32 implements the `copy trait`. This means r is still valid here

    let the_str = String::from("Haaland");
    let (the_str, len) = calculate_len(the_str);
    println!("The length of the string of {the_str} is {len}");

    //References and Borrowing example
    let s8 = String::from("Rusty");
    let s9 = calc_length(&s8);
    println!("The value of {s8} is {s9}..");

    // Mutable references example
    let mut s10 = String::from("hello");
    change(&mut s10);

    // String slices
    let s11 = String::from("hello, world");
    let hello = &s11[0..5];
    let world = &s11[6..11];
    println!("{hello}, {world}");
}

fn takes_ownership(some_string: String) { // some_string comes into scope
    println!("{some_string}");
} // Here, some_string goes out of scope and `drop` is called. The backing
    
// memory is freed.
fn makes_copy(some_integer: i32) { // some_integer comes into scope
    println!("{some_integer}");
} // Here, some_integer goes out of scope. Nothing special happens

fn calculate_len(stng: String) -> (String, usize) {
    let length = stng.len();
    (stng, length)
}

// References and Borrowing
fn calc_length (s7: &String) -> usize {
    return s7.len(); // Just reads the length, does not take ownership
}  

// Mutable References
fn change(a_string: &mut String) {
    a_string.push_str(", world");
}
