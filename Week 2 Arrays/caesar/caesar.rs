use std::io;
use std::env;
use std::process::ExitCode;

fn main() -> ExitCode {
    // get Args into a vector (one arg, a number)
    let args: Vec<String> = env::args().collect();
    if args.len() != 2 {
        println!("Please enter a key (a number > 1) when running this program. e.g. ./caesar 3");
        return ExitCode::from(1);
    }
    // Take a string, pass it down.
    let key = convert_key(&args[1]);
    let message: String = query_user("Please enter a phrase to encrypt: ");
    let cyphertext = encrypt_message(&message, &key);
    println!("Your plaintext was: {}", message);
    println!("Your cyphertext is: {}", cyphertext);
    ExitCode::SUCCESS
}

fn encrypt_message(message: &str, key: &u32) -> String {
    // Encrypt the message
    let mut cryptext: String = String::new();
    for letter in message.chars() { // Can't iterate strings so must convert to char
        let crypt_number = letter as u32 + key % 26;
        let crypt_letter = char::from_u32(crypt_number).expect("Error").to_string();
        cryptext.push_str(&crypt_letter);
    }
    return cryptext;
}

fn convert_key(key: &String) -> u32 {
    let converted_key = key.parse::<u32>();
    return converted_key.expect("Please enter a key (a number > 1) when running this program. e.g. ./caesar 3");
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