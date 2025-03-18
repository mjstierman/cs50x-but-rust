use std::io;
use std::str;
// import a Scrabble dictionary

fn main () {
    let player_1: String = query_user("Player 1: ");
    let player_2: String = query_user("Player 2: ");
    let p1_score = calc_word(&player_1);
    let p2_score = calc_word(&player_2);
    println!("Player 1 Score: {}", p1_score);
    println!("Player 2 Score: {}", p2_score);
    if p1_score > p2_score { println!("Player 1 wins!"); }
    else if p2_score > p1_score { println!("Player 2 wins!"); }
    else { println!("Uh oh! Something went wrong."); }
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

fn calc_word(word: &str) -> u32 {
    // break up string into array of chars
    // Scrabble dictionary not implemented to validate entries
    let word_chars: Vec<char> = word.chars().collect();
    // use Rust's `match` for each char
    let mut score: u32 = 0;
    for letter in word_chars {
        let letter_score = match letter {
            'A' | 'a' => 1,
            'B' | 'b' => 3,
            'C' | 'c' => 3,
            'D' | 'd' => 2,
            'E' | 'e' => 1,
            'F' | 'f' => 4,
            'G' | 'g' => 2,
            'H' | 'h' => 4,
            'I' | 'i' => 1,
            'J' | 'j' => 8,
            'K' | 'k' => 5,
            'L' | 'l' => 1,
            'M' | 'm' => 2,
            'N' | 'n' => 1,
            'O' | 'o' => 1,
            'P' | 'p' => 3,
            'Q' | 'q' => 10,
            'R' | 'r' => 1,
            'S' | 's' => 1,
            'T' | 't' => 1,
            'U' | 'u' => 1,
            'V' | 'v' => 4,
            'W' | 'w' => 4,
            'X' | 'x' => 8,
            'Y' | 'y' => 4,
            'Z' | 'z' => 10,
            _ => 0,
        };
        score += letter_score;
    }
    return score;
}