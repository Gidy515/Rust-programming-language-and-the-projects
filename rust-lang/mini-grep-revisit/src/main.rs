use std::env; // The env module provides functions for working with environment variables: Environment Variables are key-value pairs that can be used to configure the behavior of a program or provide information about the environment in which it is running. They are often used to store configuration settings, such as database connection strings, API keys, or other sensitive information that should not be hardcoded in the source code.
use std::fs; // The fs module provides functions for working with the file system, such as reading and writing files, creating directories, and managing file metadata. It allows you to perform various operations on files and directories, such as opening, reading, writing, and deleting them.

// the std::env::args().collect(); can also be used as an alternative to the env::args().collect without the need to so the `use std::env;`

fn main() {
    let args: Vec<String> = env::args().collect(); // The env::args() function returns an iterator over the command-line arguments passed to the program. The first argument (args[0]) is typically the name of the program itself, and subsequent arguments are the additional parameters provided by the user. 
    //dbg!(args); // The dbg! macro is a convenient way to print the value of an expression along with its source code location. It is often used for debugging purposes to quickly inspect the values of variables or expressions without needing to set up a more complex logging mechanism. When you use dbg!(args), it will print the contents of the args vector, which contains the command-line arguments passed to the program.

    let bin = &args[0]; // The first argument (args[0]) is typically the name of the program itself, and subsequent arguments are the additional parameters provided by the user.
    let query = &args[1]; 
    let file_path = &args[2];

    println!("Searching for {}", query);
    println!("In the file {}", file_path);
    println!("The binary name is {}", bin);

    let contents = fs::read_to_string(file_path).expect("Should have been able to read the file"); // The fs::read_to_string function reads the entire contents of a file into a String. It takes the file path as an argument and returns a Result<String, std::io::Error>. If the file is successfully read, it returns Ok(String) containing the file's contents. If there is an error (e.g., the file does not exist or cannot be read), it returns Err(std::io::Error) with details about the error. The expect method is used to handle the Result; if the Result is Err, it will panic and print the provided message.
    println!("With the content:\n{}", contents);

    let (query, file_path) = parse_config(&args);
}

fn parse_config(args: &[String]) -> (&str, &str) {
    let query = &args[1];
    let file_path = &args[2];

    (query, file_path)
}  
