use crate::garden::vegetables::Asparagus;
use std::collections::HashMap;

pub mod garden;
fn main() {
    let plant = Asparagus {
        soup: String::from("Spinach"),
    };
    println!("I'm making soup with {:?}", plant);

    let mut map = HashMap::new();
    map.insert(1, 2); // Bringing a HashMap into scope in an idiomatic way
}
