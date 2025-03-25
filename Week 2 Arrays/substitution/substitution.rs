use std::io;
use std::env;
use std::process::ExitCode;

fn main() -> ExitCode {
    // Collect the CLI args into a vector
    let args: Vec<String> = env::args().collect();
    // validate input
    if args.len() != 2 {
        println!("Please enter a key consisting of 26 letters when running this program. e.g. ./caesar ZYXWVUTSRQPONMLKJIHGFEDCBA");
        return ExitCode::from(1);
    }
    // create a key from the argument
    let key: Vec<char> = args[1].chars().collect();
    // ensure the key is long enough
    if key.len() != 26 {
        println!("Please enter a key consisting of 26 letters when running this program. e.g. ./caesar ZYXWVUTSRQPONMLKJIHGFEDCBA");
        return ExitCode::from(1);
    }
    // call fxns to encrypt and print the message + cypher
    let message: String = query_user("Please enter a phrase to encrypt: ");
    let cyphertext = encrypt_message(&message, &key);
    println!("Your plaintext was: {}", message);
    println!("Your cyphertext is: {}", cyphertext);
    ExitCode::SUCCESS
}

fn encrypt_message(message: &str, key: &Vec<char>) -> String {
    // Encrypt the message
    let mut cryptext: String = String::new();
    for letter in message.chars() {
        let crypt_letter = letter_lookup(letter, key).to_string();
        cryptext.push_str(&crypt_letter);
    }
    return cryptext;
}

fn letter_lookup(letter: char, key: &Vec<char>) -> char {
    // match the letter in the message to the index of the key
    let crypt_letter = match letter {
        'A' | 'a' => key[0],
        'B' | 'b' => key[1],
        'C' | 'c' => key[2],
        'D' | 'd' => key[3],
        'E' | 'e' => key[4],
        'F' | 'f' => key[5],
        'G' | 'g' => key[6],
        'H' | 'h' => key[7],
        'I' | 'i' => key[8],
        'J' | 'j' => key[9],
        'K' | 'k' => key[10],
        'L' | 'l' => key[11],
        'M' | 'm' => key[12],
        'N' | 'n' => key[13],
        'O' | 'o' => key[14],
        'P' | 'p' => key[15],
        'Q' | 'q' => key[16],
        'R' | 'r' => key[17],
        'S' | 's' => key[18],
        'T' | 't' => key[19],
        'U' | 'u' => key[20],
        'V' | 'v' => key[21],
        'W' | 'w' => key[22],
        'X' | 'x' => key[23],
        'Y' | 'y' => key[24],
        'Z' | 'z' => key[25],
        _ => letter,
    };
    return crypt_letter;
}

fn query_user(query: &str) -> String {
    // Function to prompt the user for input
    let query: &str = query;
    let mut response = String::new();
    
    println!("{}",query);
    io::stdin()
        .read_line(&mut response)
        .expect("Failed to read line.");

    return response;
}