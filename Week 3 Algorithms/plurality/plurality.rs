use std::io;
use std::env;
use std::str;
use std::string;
use std::process::ExitCode;

// Define maximum number of candidates
const MAX: usize = 9;

// Define the Candidate datatype
pub struct Candidate {
    name: String,
    votes: u8,
};

// Global array of candidates


fn main() -> ExitCode {
    // Collect the CLI args into a vector
    let args: Vec<String> = env::args().collect();
    // Number of candidates is equal to the number of arguments
    let candidate_count: usize = args.len() - 1;

    // Populate array of candidates
    if candidate_count > MAX {
        println!("Maximum number of candidates is {MAX} candidates.");
        return ExitCode::Failure;
    }
    if candidate_count == 0 {
        println!("Usage: plurality [candidate ...]");
        return ExitCode::USAGE;
    }

    let mut candidates: [Candidate; MAX];

    for i in 0..candidate_count {
        candidates[i] = Candidate { name: args[i + 1].clone(), votes: 0 };
    }

    // Get the number of voters from the user
    let voter_count_input:String = query_user("Number of voters: ");
    let voter_count: usize = voter_count_input.trim().parse().expect("Invalid number");

    for i in 0..voter_count {
        let name: String = query_user("Vote: ");

        if !vote(&name) {
            println!("Invalid vote.");
        }
    }

    print_winner();
    ExitCode::Success;
}

fn vote(name: &str) -> bool {

}

fn print_winner() {

}

fn query_user(query: &str) -> String {
    // Function to prompt the user for input
    let mut response = String::new();

    println!("{}",query);
    io::stdin()
        .read_line(&mut response)
        .expect("Failed to read line.");

    return response;
}
