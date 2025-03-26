use std::io;
use std::str;

// Max number of candidates
let pub MAX: u32 = 9;

// preferences[i][j] is number of voters who prefer i over j
let mut preferences[[u32; MAX]; MAX] = [][];

// locked[i][j] means i is locked in over j
let mut locked[[bool; MAX]; MAX] = [][];

// Each pair has a winner, loser
struct pair
{
    pub winner: u32;
    pub loser: u32;
}

// Array of candidates
let mut candidates: Vec<candidate> = Vec::with_capacity(MAX);
pair pairs[MAX * (MAX - 1) / 2];

pub mut pair_count: u32;
pub mut candidate_count: u32;

// Main Program

fn main() -> u32 {
    // Check for invalid usage
    let args: Vec<String> = env::args().collect();
    if (args < 2) {
        println!("Usage: tideman [candidate ...]");
        return ExitCode::from(1);
    }

    // Populate array of candidates
    candidate_count = args - 1;
    if (candidate_count > MAX) {
        println!("Maximum number of candidates is {}", MAX);
        return ExitCode::from(2);
    }

    let i: u32 = 0;
    while i < candidate_count {
        candidates[i] = argv[i + 1];
        i+=1;
    }

    // Clear graph of locked in pairs
    i = 0;
    while i < candidate_count {
        let j: u32 = 0;
        while j < candidate_count
        {
            locked[i][j] = false;
            j += 1;
        }
        i += 1;
    }

    pair_count = 0;
    let voter_count: u32 = get_int("Number of voters: ");

    // Query for votes
    i = 0;
    while i < voter_count {
        // ranks[i] is voter's ith preference
        let mut candidates: Vec<u32> = Vec::with_capacity(candidate_count);

        // Query for each rank
        let j: u32 = 0;
        while j < candidate_count {
            string name = get_string("Rank %i: ", j + 1);

            if (!vote(j, name, ranks))
            {
                printf("Invalid vote.\n");
                return 3;
            }
            j += 1;
        }

        record_preferences(ranks);
        i += 1;
    }

    add_pairs();
    sort_pairs();
    lock_pairs();
    print_winner();
    return 0;
}

// Update ranks given a new vote
fn vote(rank: u32, name: String, ranks: Vec<u32>) -> bool {
    // TODO
    return false;
}

// Update preferences given one voter's ranks
fn record_preferences(ranks: Vec<u32>) {
    // TODO
    return;
}

// Record pairs of candidates where one is preferred over the other
fn add_pairs() {
    // TODO
    return;
}

// Sort pairs in decreasing order by strength of victory
fn sort_pairs() {
    // TODO
    return;
}

// Lock pairs into the candidate graph in order, without creating cycles
fn lock_pairs()
{
    // TODO
    return;
}

// Print the winner of the election
fn print_winner()
{
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