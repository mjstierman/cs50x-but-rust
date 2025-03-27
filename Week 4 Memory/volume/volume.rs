use std::env;
use std::fs;
use std::io;
use std::process::ExitCode;

// Number of bytes in .wav header
const HEADER_SIZE: u32 = 44;

fn main() -> ExitCode {
    // Check command-line arguments
    let args: Vec<String> = env::args().collect();
    if args.len() != 4 {
        println!("Usage: ./volume input.wav output.wav factor\n");
        return ExitCode::from(1);
    }

    // Open the input file
    let mut file = match fs::File::open(&args[1]) {
        Ok(file) => file,
        Err(e) => {
            println!("Error opening file: {}", e);
            return ExitCode::from(1);
        }
    };
    
    // TODO: Copy header from input file to output file
    // TODO: Read samples from input file and write updated data to output file

    return ExitCode::SUCCESS;
}
