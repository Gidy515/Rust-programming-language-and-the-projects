Four important details to notice about the "println!("Hello world");" line

1. Rust style is to indent with 4 spaces, not a tab.

2. println! calls a macro, not a function. If it were a function, it could have just been println!. The ! indicates its a macro you are calling, macros and normal functions don't always follow the same rules.

3. The "Hello, world" string is passed as an argument to println! and the string is printed to the screen.

4. The ; indicates the end to that line of code.
