// Get a user's name and greet them

use std::io; // optional, you can still call with std::io::<fxn>

fn main(){
    // ! below means this is a macro, not a fxn
    println!("Please state your name: ");

    // declare new, mutible variable, and assign a new instance of String type
    let mut name = String::new();
    
    // call stdin fxn from io module
    io::stdin()
        // _reference_ (not copy!) input and assigns to var name
        .read_line(&mut name) 
        // if Result is an Err value, expect will cause the program to crash
        .expect("Failed to read line.");

    // "think of {} as little crab pincers that hold a value in place"
    println!("Hello, {}", name);
}