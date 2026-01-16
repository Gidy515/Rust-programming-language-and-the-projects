fn main() {
    // Storing UTF-8 Encoded Text with Strings
    // Creating a new String
    let mut s = String::new();

    let data = "Initial data";
    let st = data.to_string();

    // valid strings yeah..
    /*let hello = String::from("ﻋﻠﯿﻜﻢ");
    let hello = String::from("Dobrý den");
    let hello = String::from("Hello");
    let hello = String::from("םוָֹלשׁ");
    let hello = String::from("नमे");
    let hello = String::from("こんにちは");
    let hello = String::from("안녕하세요");
    let hello = String::from("你好");
    let hello = String::from("Olá");
    let hello = String::from("Здравствуйте");
    let hello = String::from("Hola");*/

    // Updating a String
    let mut t = String::from("foo");
    t.push_str("bar"); // foobar

    let mut s1 = String::from("foo");
    let s2 = "bar";
    s1.push_str(s2);
    println!("s1 is {s1}"); 

    // Concatenation with the + Operator or the format! Macro
    let s1 = String::from("Hello, ");
    let s2 = String::from("world!");
    /*
    The string s3 will contain Hello, world!. The reason s1 is no longer valid after the addition, and the reason we used a reference to s2, 
    has to do with the signature of the method that’s called when we use the + operator. The + operator uses the add method, whose signature 
    looks something like this:
    fn add(self, s: &str) -> String {
     */
    let s3 = s1 + &s2;
    println!("{s3}");

    let s4 = String::from("tik");
    let s5 = String::from("tac");
    let s6 = String::from("toe");
    let s7 = format!("{s4}-{s5}-{s6}");
    println!("{s7}");

    // Slicing Strings
    /*let ello = "Здравствуйте";
    let sr = &ello[0..4]; panic*/
    /*
    Methods for Iterating Over Strings The best way to operate on pieces of strings is to be explicit about whether you want characters 
    or bytes. For individual Unicode scalar values, use the chars method. Calling chars on “Зд” separates out and returns two values of 
    type char, and you can iterate over the result to access each element:
     */ 
    for c in "Зд".chars() {
        println!("{c}");
    }
    // Alternatively, the bytes method returns each raw byte, which might be appropriate for your domain:
    for b in "Зд".bytes() {
        println!("{b}");
    }
}
