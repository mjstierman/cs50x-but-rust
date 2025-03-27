use std::env;
use std::fs;
use std::process::ExitCode;

// Number of bytes in .wav header
const HEADER_SIZE: u32 = 44;

fn main() -> ExitCode {
    // Check command-line arguments
    let args: Vec<String> = env::args().collect();
    if args.len() != 4
    {
        println!("Usage: ./volume input.wav output.wav factor\n");
        return ExitCode::from(1);
    }

    // Open files and determine scaling factor
    let file = fs::read(&args[1]);
    
    //let mut output = fs::write(&args[2]).expect("Could not open file.");
    let factor: f32 = args[3].trim().parse().expect("Not a number.");

    // TODO: Copy header from input file to output file

    // TODO: Read samples from input file and write updated data to output file

    // No need to close files in Rust

    return ExitCode::SUCCESS
}

