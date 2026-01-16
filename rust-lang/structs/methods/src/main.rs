#[derive(Debug)]
struct Rectangle3 {
    height: u32,
    width: u32,
}

impl Rectangle3 {
    fn area (&self) -> u32 {
        self.height * self.width
    }
}

// Methods with more parameters
/*
 implementing a second method on the Rectangle struct. This time we want an instance of Rectangle to
take another instance of Rectangle and return true if the second Rectangle can fit completely within self (the first Rectangle);
otherwise, it should return false. That is, once we’ve defined the can_hold method, we want to be able to write the program shown
 */
//#[derive(Debug)]
struct Rectangle4 {
    height: u32,
    width: u32,
}

impl Rectangle4 {
    fn area2(&self) -> u32 {
        self.width * self.height
    }
    fn can_hold(&self, other: &Rectangle4) -> bool {
        self.width > other.width && self.height > other.height
    }
}

//Associated functions

fn main() {
    let rect3 = Rectangle3 {
        height: 50,
        width: 35,
    };

    println!("The are of the rectangle is {} and the struct is {:#?}", rect3.area(), rect3);

    // Methods with more parameters (function call)
    let rect4 = Rectangle4 {
        width: 30,
        height: 50,
    };
    
    let rect5 = Rectangle4 {
        width: 10,
        height: 40,
    };
    let rect6 = Rectangle4 {
        width: 60,
        height: 45,
    };
    println!("Can rect1 hold rect2? {}", rect4.can_hold(&rect5));
    println!("Can rect1 hold rect3? {}", rect4.can_hold(&rect6));
}
