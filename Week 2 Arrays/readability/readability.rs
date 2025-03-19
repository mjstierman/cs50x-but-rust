use std::io;
use std::char;

fn main() {
    let text: String = query_user("Please enter a text to evaluate: ");
    let score: f32 = get_index(count_words(&text), count_letters(&text), count_sentences(&text));
    get_grade_level(&score);
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

fn count_sentences(phrase: &str) -> f32 {
    // Split the sentences in the phrase into a vector and count them
    let sentence_list: Vec<&str> = phrase.split_terminator(". ").collect();
    let sentence_count: f32 = sentence_list.len() as f32;
    return sentence_count;
}

fn count_words(phrase: &str) -> f32 {
    // Split the words in the phrase into a vector and count them
    let word_list: Vec<&str> = phrase.split_whitespace().collect();
    let word_count: f32 = word_list.len() as f32;
    return word_count;
}

fn count_letters(phrase: &str) -> f32 {
    // Split the letters in the phrase into a vector and count them
    let letters: Vec<char> = phrase.trim().chars().collect();
    let letter_count: f32 = letters.len() as f32;
    return letter_count;
}

fn get_index(words: f32, letters: f32, sentences: f32) -> f32 {
    // L = average number of letters per 100 words
    let l: f32 = letters / words * 100.0;
    // S = average number of sentences per 100 words
    let s: f32 = sentences / words * 100.0;
    // Turn input string into a vector of sentences, count, divide by 100
    let index: f32 = 0.0588 * l - 0.296 * s - 15.8;
    return index;
}

fn get_grade_level(index: &f32) {
    // Round the index to a whole number and ballpark the grade level
    let index = &index.round();
    if *index <= 1.0 { println!("Before Grade 1"); }
    else if *index >= 16.0 { println!("Grade 16+"); }
    else { println!("Grade {}", *index); }
}