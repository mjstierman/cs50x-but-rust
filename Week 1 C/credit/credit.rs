use std::io;
use std::str;

fn main() {
    let mut unvalidated_ccn = query_user("Enter Credit Card Number to verify:"); 
    while unvalidated_ccn.chars().count() > 17 || unvalidated_ccn.chars().count() < 12 {
        unvalidated_ccn = query_user("Enter Credit Card Number to verify:");
    }
    if check_digit(&unvalidated_ccn) {
        id_bank(&unvalidated_ccn);
    }
    else if check_digit(&unvalidated_ccn) == false {
        println!("Validation Failed.");
    }
    else {
        println!("An unknown error occurred.");
    }
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

fn check_digit(ccn_to_check: &str) -> bool {
    // Break up string into array of chars to make it easier to work with.
    let mut ccn_array: Vec<char> = ccn_to_check.trim().chars().collect();
    let mut check: u32 = 0;
    
    // Multiply every other digit by 2, starting with the number’s second-to-last digit, and then add those products’ digits together.
    // Using some math shortcuts to faithfully answer the problem, just not exactly as described.
    for count in (0.. ccn_array.len()-1).rev().step_by(2) {
        let ccn_num = ccn_array[count].to_digit(10).unwrap();
        check += if ccn_num * 2 < 10 { ccn_num*2 } else { (ccn_num*2) - 9 };
        // remove the number after being added
        ccn_array.remove(count);
    }

    // Add the sum to the sum of the digits that weren’t multiplied by 2.
    for num in 0..ccn_array.len() {
        check += ccn_array[num].to_digit(10).unwrap();
    }

    // If the total’s last digit is 0 (or, put more formally, if the total modulo 10 is congruent to 0), the number is valid
    return check % 10 == 0
}

fn id_bank(valid_ccn: &str) {
    let first_two_digits: String = valid_ccn.chars().take(2).collect();
    // American Express numbers start with 34 or 37
    let amex_starts = ["34", "37"];
    // Visa numbers start with 4
    let visa_starts = ["40", "41", "42", "43", "44", "45", "46", "47", "48", "49"];
    // MasterCard numbers start with 51, 52, 53, 54, or 55
    let mc_starts = ["51", "52", "53", "54", "55"];

    if amex_starts.contains(&first_two_digits.as_str()) {
        println!("American Express");
    } else if visa_starts.contains(&first_two_digits.as_str()) {
        println!("Visa");
    } else if mc_starts.contains(&first_two_digits.as_str()) {
        println!("MasterCard");
    } else {
        println!("Not a known card.");
    }
}

