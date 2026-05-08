use std::env; // The env module provides functions for working with environment variables: Environment Variables are key-value pairs that can be used to configure the behavior of a program or provide information about the environment in which it is running. They are often used to store configuration settings, such as database connection strings, API keys, or other sensitive information that should not be hardcoded in the source code.
use std::process; // The process module provides functions for working with processes, such as spawning new processes, managing child processes, and handling process-related tasks. It allows you to execute external commands, manage the lifecycle of processes, and interact with the operating system's process management features. In this code, it is used to exit the program with a specific status code when there is an error parsing the command-line arguments. 

use mini_grep_revisit::Config;

fn main() {
    let args: Vec<String> = env::args().collect(); // The env::args() function returns an iterator over the command-line arguments passed to the program. The first argument (args[0]) is typically the name of the program itself, and subsequent arguments are the additional parameters provided by the user. 
    //dbg!(args); // The dbg! macro is a convenient way to print the value of an expression along with its source code location. It is often used for debugging purposes to quickly inspect the values of variables or expressions without needing to set up a more complex logging mechanism. When you use dbg!(args), it will print the contents of the args vector, which contains the command-line arguments passed to the program.
    //let (query, file_path) = parse_config(&args);
    let config = Config::build(&args).unwrap_or_else(|err|{
        eprintln!("Problem parsing arguments: {err}");
        process::exit(1);
    });

    println!("Searching for {}", config.query);
    println!("In the file {}", config.file_path);

    //let bin = &args[0]; // The first argument (args[0]) is typically the name of the program itself, and subsequent arguments are the additional parameters provided by the user.
    //let query = &args[1]; 
    //let file_path = &args[2];

    if let Err(e) = mini_grep_revisit::run(config) {
        eprintln!("Application error: {e}");
        process::exit(1);
    }

    // run(config);
}
