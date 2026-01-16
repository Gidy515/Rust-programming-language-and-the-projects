struct User {
    active: bool,
    username: String,
    email: String,
    sign_in_count: u64,
}

struct Color (i32, i32, i32);
struct Point(i32, i32, i32);

//tuple example for to calculate area of a rectangle
fn area (dimensions: (u32, u32)) -> u32 {
    dimensions.0 * dimensions.1
}

// Refactoring with Structs: Adding More Meaning, using our rectangle calculation example to add descriptive meaning 
#[derive(Debug)] 
struct Rectangle {
    width: u32,
    height: u32,
}
fn struct_area_calc (rectangle: &Rectangle) -> u32 {
    rectangle.width * rectangle.height
}

fn main() {
    let mut user1 = User {
        active: true,
        username: String::from("Braunt Haaland"),
        email: String::from("haaland@gmail.com"),
        sign_in_count: 8,
    };

    user1.username = String::from("Alexander Isak"); // changing a struct's field

    println!("User's username is {}, User is {}, User email is {}, User signed in {} times", user1.username, user1.active, user1.email, user1.sign_in_count);

    // creating another instance of a struct using values of the fields from another instance
    let user2 = User {
        active: user1.active,
        username: user1.username,
        email: user1.email,
        sign_in_count:user1.sign_in_count,
    };

    // Using Tuple Structs Without Named Fields to Create Different Types
    let black = Color(0, 0, 0);
    let origin = Point(0, 0, 0);
    println!("{} {}", black.2, origin.1);

    //tuple example for to calculate area of a rectangle (function call)
    let rectangle1 = (30, 50);
    println!("The area of the rectangle is {}", area(rectangle1));

    // Refactoring with Structs: Adding More Meaning, using our rectangle calculation example to add descriptive meaning (function call)
    let rectangle2 = Rectangle {
        height: 40,
        width: 60,
    };
    println!("The area of the rectangle is {}", struct_area_calc(&rectangle2));
    /* Printing the struct, Rectangle using the #[derive(Debug)] which is created at the Rectangle struct 
    because Rust does not implement std::fmt::Display (which is default with the println! macro), 
    and does not work with the {:?} and {:#?} as well without the trait*/
    println!("The rectangle {:#?}", rectangle2);
    /*
    Calling the dbg! macro prints to the standard error console
    stream (stderr), as opposed to println!, which prints to the
    standard output console stream (stdout). 
     */
    dbg!(&rectangle2); // we don't want dbg! to take ownership of rectangle2, so we use a reference
}
