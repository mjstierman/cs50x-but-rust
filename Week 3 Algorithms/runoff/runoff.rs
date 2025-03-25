use std::io;
use std::str;
use std::str::FromStr;

// Max voters and candidates
let MAX_VOTERS: u32 = 100
let MAX_CANDIDATES: u32 = 9

// preferences[i][j] is jth preference for voter i
int preferences[MAX_VOTERS][MAX_CANDIDATES];

// Candidates have name, vote count, eliminated status
struct candidate {
    pub name: String;
    pub votes: u32;
    pub eliminated: bool;
}

// Array of candidates
let mut candidates: Vec<candidate> = Vec::with_capacity(MAX_CANDIDATES);

// Numbers of voters and candidates
let mut voter_count: u32;
let mut candidate_count: u32;

fn main() -> ExitCode {
    // Check for invalid usage
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        println!("Usage: runoff [candidate ...]");
        return ExitCode::from(1);
    }

    // Populate array of candidates
    candidate_count = args - 1; // does this need to be `args.len() - 1`?
    if (candidate_count > MAX_CANDIDATES)
    {
        println!("Maximum number of candidates is {}", MAX_CANDIDATES);
        return ExitCode::from(2);
    }
    for i in candidates {
        candidates[i].name = args[i + 1];
        candidates[i].votes = 0;
        candidates[i].eliminated = false;
    }
    
    voter_count = get_int("Number of voters: ");
    if (voter_count > MAX_VOTERS)
    {
        println!("Maximum number of voters is %i\n", MAX_VOTERS);
        return ExitCode::from(3);
    }

    // Keep querying for votes
    let mut i: u32 = 0;
    while i < voter_count {
        // Query for each rank
        let mut j: u32 = 0;
        while j < candidate_count {
            let name: String = get_string("Rank {}: ", i+1); // not sure this is going to work

            // Record vote, unless it's invalid
            if (!vote(i, j, name))
            {
                println!("Invalid vote.\n");
                return ExitCode::from(4);
            }
        }
        i += i;
    }

    // Keep holding runoffs until winner exists
    while (true) {
        // Calculate votes given remaining candidates
        tabulate();

        // Check if election has been won
        let won: bool = print_winner();
        if (won) {
            break;
        }

        // Eliminate last-place candidates
        let min: u32 = find_min();
        let tie: bool = is_tie(min);

        // If tie, everyone wins
        if (tie) {
            let i: u32 = 0;
            while i < candidate_count {
                if (!candidates[i].eliminated) {
                    println!("{}", candidates[i].name);
                    i += 1;
                }
            }
            break;
        }

        // Eliminate anyone with minimum number of votes
        eliminate(min);

        // Reset vote counts back to zero
        let i: u32 = 0;
        while i < candidate_count {
            candidates[i].votes = 0;
            i += 1;
        }
    }
    return 0;
}

// Record preference if vote is valid
fn vote(int voter, int rank, string name) -> bool {
    // TODO
    return false;
}

// Tabulate votes for non-eliminated candidates
fn tabulate() {
    // TODO
    return;
}

// Print the winner of the election, if there is one
fn print_winner() -> bool {
    // TODO
    return false;
}

// Return the minimum number of votes any remaining candidate has
fn find_min() -> u32 {
    // TODO
    return 0;
}

// Return true if the election is tied between all candidates, false otherwise
fn is_tie(min: u32) -> bool {
    // TODO
    return false;
}

// Eliminate the candidate (or candidates) in last place
fn eliminate(min: u32) {
    // TODO
    return;
}

fn get_string(query: &str) -> String {
    // Function to prompt the user for String input
    let query: &str = query;
    let mut response = String::new();
    
    println!("{}",query);
    io::stdin()
        .read_line(&mut response)
        .expect("Failed to read line.");

    return response;
}

fn get_int(query: &str) -> u32 {
    // Function to prompt the user for numerical input
    let query: &str = query;
    let mut response = String::new();
    
    println!("{}",query);
    io::stdin()
        .read_line(&mut response)
        .expect("Failed to read line.");

    let int = u32::from_str(response)
        .unwrap()
        .expect("Failed to convert string to u32");

    return int;
}