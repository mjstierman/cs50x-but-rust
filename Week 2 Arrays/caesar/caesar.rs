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
    // convert arg from String to u32
    let key_char = args[1].char();
    let key: u32 = to_u32(key_char);

    // Take a string, pass it down.
    let message: String = query_user("Please enter a phrase to encrypt: ");
    let cyphertext = encrypt_message(message, &key);
    println!("Your plaintext was: {}", message);
    println!("Your cyphertext is: {}", cyphertext);
    ExitCode::SUCCESS
}

fn encrypt_message(message: String, key: &u32) -> String {
    // for each letter in the message, convert to an int, apply the key
    // convert back into a char, and add to encrypted message
    let mut cryptext: String = String::new();
    let message_array: Vec<char> = message.chars().collect();
    let k = key;
    let mut index = 0;
    while index < message_array.len() {
        let mut p = message_array[index];
        let mut p = to_u32(p);
        let mut c = (p + k) % 26;
        c = to_char(&c);
        cryptext.push_str(c);
        index += 1;
    }
    // return the assembled encrypted text
    return cryptext;
}

fn to_u32(&letter: char) -> u32 {
    // convert A-Z and a-z chars into an int
    let char_to_convert: char = letter.chars();
    let letters: [char; 52] = [
        'A', 'B', 'C', 'D', 'E', 'F', 'G', 'H', 'I', 'J', 
        'K', 'L', 'M', 'N', 'O', 'P', 'Q', 'R', 'S', 'T', 
        'U', 'V', 'W', 'X', 'Y', 'Z', 'a', 'b', 'c', 'd', 
        'e', 'f', 'g', 'h', 'i', 'j', 'k', 'l', 'm', 'n', 
        'o', 'p', 'q', 'r', 's', 't', 'u', 'v', 'w', 'x', 
        'y', 'z',
    ];
    let mut index: u32 = 0;
    while index < letters.len() {
        if  letters[index] == char_to_convert {
            return index as u32;
        }
        else { 
            index += 1;
        }
    }
}

fn to_char(int: &u32) -> char {
    // convert an int into chars A-Z and a-z
    let int_to_convert = int;
    let letters: [char; 52] = [
        'A', 'B', 'C', 'D', 'E', 'F', 'G', 'H', 'I', 'J', 
        'K', 'L', 'M', 'N', 'O', 'P', 'Q', 'R', 'S', 'T', 
        'U', 'V', 'W', 'X', 'Y', 'Z', 'a', 'b', 'c', 'd', 
        'e', 'f', 'g', 'h', 'i', 'j', 'k', 'l', 'm', 'n', 
        'o', 'p', 'q', 'r', 's', 't', 'u', 'v', 'w', 'x', 
        'y', 'z',
    ];
    return letters[int_to_convert];
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