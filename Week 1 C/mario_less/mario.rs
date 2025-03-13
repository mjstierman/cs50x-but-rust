// import library for accepting input from users
use std::io;

fn main() {
    let answer = query_user();
    print_rows(answer);
}

// ask the user for a number 1-10... and make sure they listen.
fn query_user() -> u8 {
    // initialize answer as 0 to start the prompt
    let mut jumps = 0;

    // any reponse other than 1..10 prompts the prompt
    while jumps < 1 || jumps > 10 { 
        println!("How high should Mario jump (1-10)? ");
        
        let mut response = String::new();

        io::stdin()
            .read_line(&mut response)
            .expect("Failed to read line.");

        // convert string response to an int 
        jumps = response
            .trim()
            .parse()
            .expect("Input not a number");
    }

    return jumps;
}

// fxn to print the rows
// "In function signatures, you must declare the type of each parameter."
fn print_rows(input: u8) {
    let rows = input;
    let mut row = 0;
    
    while row < rows {
        let mut spaces = 0;
        while spaces < rows-row-1 {
            print!(" ");
            spaces += 1;
        };
        let mut hashes = 0;
        while hashes <= row {
            print!("#");
            hashes += 1;
        };
        println!();
        row += 1;
    } 
}