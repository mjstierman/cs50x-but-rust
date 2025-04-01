use std::env;
use std::fs;
use std::io::Write;
use std::process::ExitCode;

// Number of bytes in .wav header
const HEADER_SIZE: usize = 44;

fn main() -> ExitCode {
    // Check command-line arguments
    let args: Vec<String> = env::args().collect();
    if args.len() != 4 {
        println!("Usage: ./volume input.wav output.wav factor\n");
        return ExitCode::from(1);
    }
    
    // Validate third arg is an int
    let factor: u8 = match args[3].trim().parse() {
        Ok(factor) => factor,
        Err(e) => {
            println!("Error getting factor: {}", e);
            return ExitCode::from(1);
        }
    };

    // Open the input file
    let input_file: Vec<u8> = match fs::read(&args[1]) {
        Ok(input_file) => input_file,
        Err(e) => {
            println!("Error opening file: {}", e);
            return ExitCode::from(1);
        }
    };

    // Create the output file
    let mut output_file = match fs::File::create_new(&args[2]) {
        Ok(output_file) => output_file,
        Err(e) => {
            println!("Error creating new file: {}", e);
            return ExitCode::from(1);
        }
    };

    // TODO: Copy header from input file to output file
    output_file.write_all(&input_file[..HEADER_SIZE]).expect("Failed to write header.");

    // Read samples from input file and write updated data to output file
    let samples = &input_file[HEADER_SIZE..];
    let updated_samples: Vec<u8> = samples.iter()
        .map(|&byte| byte.wrapping_add(factor)) // Modify each byte
        .collect();
    output_file.write_all(&updated_samples).expect("Failed to write header.");

    return ExitCode::SUCCESS;
}