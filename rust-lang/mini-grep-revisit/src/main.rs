use std::env; // The env module provides functions for working with environment variables: Environment Variables are key-value pairs that can be used to configure the behavior of a program or provide information about the environment in which it is running. They are often used to store configuration settings, such as database connection strings, API keys, or other sensitive information that should not be hardcoded in the source code.
use std::fs; // The fs module provides functions for working with the file system, such as reading and writing files, creating directories, and managing file metadata. It allows you to perform various operations on files and directories, such as opening, reading, writing, and deleting them.
use std::process; // The process module provides functions for working with processes, such as spawning new processes, managing child processes, and handling process-related tasks. It allows you to execute external commands, manage the lifecycle of processes, and interact with the operating system's process management features. In this code, it is used to exit the program with a specific status code when there is an error parsing the command-line arguments. 
use std::error::Error;

// the std::env::args().collect(); can also be used as an alternative to the env::args().collect without the need to so the `use std::env;`

fn main() {
    let args: Vec<String> = env::args().collect(); // The env::args() function returns an iterator over the command-line arguments passed to the program. The first argument (args[0]) is typically the name of the program itself, and subsequent arguments are the additional parameters provided by the user. 
    //dbg!(args); // The dbg! macro is a convenient way to print the value of an expression along with its source code location. It is often used for debugging purposes to quickly inspect the values of variables or expressions without needing to set up a more complex logging mechanism. When you use dbg!(args), it will print the contents of the args vector, which contains the command-line arguments passed to the program.
    //let (query, file_path) = parse_config(&args);
    let config = Config::build(&args).unwrap_or_else(|err|{
        println!("Problem parsing arguments: {err}");
        process::exit(1);
    });

    println!("Searching for {}", config.query);
    println!("In the file {}", config.file_path);

    //let bin = &args[0]; // The first argument (args[0]) is typically the name of the program itself, and subsequent arguments are the additional parameters provided by the user.
    //let query = &args[1]; 
    //let file_path = &args[2];

    run(config);
}

fn run(config: Config) -> Result<(), Box<dyn Error>> { 
    let contents = fs::read_to_string(config.file_path)?;
    //.expect("Should have been able to read the file"); // The fs::read_to_string function reads the entire contents of a file into a String. It takes the file path as an argument and returns a Result<String, std::io::Error>. If the file is successfully read, it returns Ok(String) containing the file's contents. If there is an error (e.g., the file does not exist or cannot be read), it returns Err(std::io::Error) with details about the error. The expect method is used to handle the Result; if the Result is Err, it will panic and print the provided message.
    println!("With the content:\n{}", contents);
    Ok(())
}

struct Config {
    query: String,
    file_path: String,
}

impl Config {
    fn build(args: &[String]) -> Result<Config, &'static str> {
        if args.len() < 3 {
            return Err("Not enough arguments");
        }
        let query = args[1].clone();
        let file_path = args[2].clone();

        Ok(Config {query, file_path})
    }
}