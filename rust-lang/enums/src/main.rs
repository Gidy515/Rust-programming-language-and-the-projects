enum IpAddrKind {
    v4,
    v6,
}

enum IpAddrWithData {
    V4(u8, u8, u8, u8),
    V6(String),
}

// A Message enum whose variants each store different amounts and types of values
enum Message {
    Quit,
    Move { x: i32, y: i32 },
    Write(String),
    ChangeColor(i32, i32, i32),
}

struct QuitMessage; // unit struct
struct MoveMessage {
    x: i32,
    y: i32,
}
struct WriteMessage(String); // tuple struct
struct ChangeColorMessage(i32, i32, i32); // tuple struct

fn route(ip_kind: IpAddrKind) {}

// The Option Enum and it's advantages over Null values
/*
 Rust does not have nulls, but it does have an enum that can encode the concept of a value being present or absent. This enum is Option<T>, 
 and it is defined by the standard library as follows:
 */
enum MyOption<T> {
    Some(T),
    None,
}

// Match expression
enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter,
}

fn main() {
    println!("ripping enums!");

    let home = IpAddrWithData::V4(127, 0, 0, 1);
    let loopback = IpAddrWithData::V6(String::from("::1"));
    
    let four = IpAddrKind::v4;
    let six = IpAddrKind::v6;

    route(IpAddrKind::v4);
    route(IpAddrKind::v4);

    let some_number = Some(90);
    let some_char = 'e';
    
    //let absent_number: Option<i32> = None;

    // Match expression function example
    fn value_in_cents(coin: Coin) -> u8 {
        match coin {
            Coin::Dime => 10,
            Coin::Nickel => 5,
            Coin::Penny => { // running multiple lines of code in a match statement requires a curly bracket & the follow up comma is optional
                println!("Lucky penny");
                1
            },
            Coin::Quarter => 25,
        }
    }

    // Matching with Option<T>
/*
Let’s say we want to write a function that takes an Option<i32> and, if there’s a value inside, adds 1 to that value. 
If there isn’t a value inside, the function should return the None value and not attempt to perform any operations.
 */
    fn option_plus_one(zee: std::option::Option<i32>) -> std::option::Option<i32> {
            match zee {
                None => None,
                Some(i) => Some(i + 1),
            }
    }

    let five = Some(5);
    let six = option_plus_one(five);
    let none = option_plus_one(None);
    println!("six: {:?}, none: {:?}", six, none);

    /*
    Rust has a pattern we can use when we want a catch-all but don’t want to use the value in the catch-all pattern: 
    _ is a special pattern that matches any value and does not bind to that value. This tells Rust we aren’t going to use the value, 
    so Rust won’t warn us about an unused variable.
     */
    let dice_roll = 9;
    match dice_roll {
        3 => add_fancy_hat(),
        7 => remove_fancy_hat(),
        _ => reroll(),
    }
    fn add_fancy_hat() {}
    fn remove_fancy_hat() {}
    fn reroll() {}

    // if let syntax as alternative to match when you only care about one pattern
    let some_u8_value = Some(0u8);
    match some_u8_value {
        Some(3) => println!("three"),
        _ => (),
    }
    if let Some(3) = some_u8_value {
        println!("three");
    }

}
