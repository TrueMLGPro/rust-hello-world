use std::io::{self, Write};
use text_io::read;
extern crate dont_disappear;

fn flush() {
    match io::stdout().flush() {
        Ok(_) => (),
        Err(error) => println!("{}", error)
    }
}

fn main() {
    println!("Hello World!");
    print!("Please input your name: ");
    // Flush the standard output stream
    flush();
    // Read input and store the value in your_name string variable
    let your_name: String = read!();
    // Print specified name
    println!("Your name is {}\n", your_name);
    dont_disappear::any_key_to_continue::custom_msg("Press Enter to continue...");
}
