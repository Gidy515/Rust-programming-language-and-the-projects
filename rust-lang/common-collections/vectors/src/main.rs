fn main() {
    // Storing Lists of Values with Vectors Vec<T>
    // creating a new vector
    let mut v: Vec<i32> = Vec::new();
    let _w = vec![1, 2, 3];

    v.push(10);
    v.push(11);
    v.push(50);
    v.push(3);

    let a = vec![2, 4, 6, 8, 10];

    let third = &a[3];
    println!("Third is {third}");

    let getter = &a.get(2); // Useful if someone enters an index beyond the vector
    match getter {
        Some(getter) => println!("We have {getter}"),
        None => println!("Nothing to print"),
    }

    /*
    // won't compile/work
    let mut v = vec![1, 2, 3, 4, 5];
    let first = &v[0];
    v.push(6);
    println!("The first element is: {first}"); // Attempting to add an element to a vector while holding a reference to an item
     */

    // iterating over the values in a Vector
    let b = vec![60, 50, 40, 30, 20, 10];
    for item in &b {
        println!("{item}");
    }

    //We can also iterate over mutable references to each element in amutable vector in order to make changes to all the elements. The for
    //loop below will add 50 to each element.
    let mut vee = vec![2, 20, 30];
    for i in &mut vee {
        *i += 50;
        println!("{i}");
    }

    // Using an Enum to store multiple types
    #[derive(Debug)]
    enum SpreadsheetCell {
        Int(i32),
        Float(f64),
        Text(String),
    }

    // Defining an enum to store values of different types in one vector
    let row = vec![
        SpreadsheetCell::Int(3),
        SpreadsheetCell::Float(5.55),
        SpreadsheetCell::Text(String::from("golden")),
    ];
    let int_of_row = &row[0];
    let float_of_row = &row[1];
    let string_of_row = &row[2];
    println!("{:#?}", int_of_row);
    println!("{:#?}", float_of_row);
    println!("{:#?}", string_of_row);

   /*  // Dropping a Vector Drops Its Elements
    {
        let v = vec![1, 2, 3, 4];
        // do stuff with v
    } // <- v goes out of scope and is freed here
     */
}