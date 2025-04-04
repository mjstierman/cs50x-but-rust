// MJS 2025-04-03: Adapted from C to Rust

use getopts::{Options, optflag, reqopt, getopts};
use std::env;
use std::fs::File;
use std::io::Read;
use std::process::ExitCode;
use std::mem;
use std::str;

use crate::bmp::BITMAPINFOHEADER;
use crate::bmp::BITMAPFILEHEADER;


mod helpers;
mod bmp;

fn print_usage(program: &str) {
    let brief = format!("Usage: {} [flag] infile outfile", program);
    println!("{}", &brief);
}

fn main() -> ExitCode {

    let args: Vec<String> = env::args().map(|x| x.to_string()).collect();
    let program = args[0].clone();

    // Define allowable filters
    // optflag arguments are: shortname, long name, description/help
    let opts = Options::new()
        .optflag("b", "blur", "Use box blur to soften an image")
        .optflag("g", "greyscale", "Convert an image to black and white")
        .optflag("r", "reflection",
                "The resulting image is what you would get by placing the original image in front of a mirror.")
        .optflag("s", "sepia",
                "Give an image an old-timey feel by making the whole image look a bit reddish-brown.")
        .reqopt("i","input","Input file", "FILE")
        .reqopt("o","output","Output file", "FILE");

    // Get filter flag and check validity
    let matches =  opts.expect("REASON").parse(&args[1..])

    // Ensure filter present
    // This actually ensures there are no "free" args passed
    if !matches.free.is_empty() {
        print_usage(&program);
        return ExitCode::from(2);
    }

    // create an array of valid flags
    let valid_args = ['b', 'g', 'r', 's'];
    // Ensure only one filter
    if !valid_args.contains(&matches[1]) {
        print_usage(&program);
        return ExitCode::from(3);
    }

    // Ensure proper usage
    // Okay "ensure proper usage" is pretty vague
    // This compares the number of args passed, and checks if it is NOT equal
    // to how many there should be (in this case, 4: self, the flag, in, & out)
    if matches.len() != 4 {
        print_usage(&program);
        return ExitCode::from(4);
    }

    // Remember filenames
    let input = matches.opt_str("i");
    let output = matches.opt_str("o");

    // Open input file
    let mut inptr = File::open(input);
    let mut buffer = Vec::new();
    if let Err(e) = input.read_to_end(&mut buffer) {
        eprintln!("Error reading file: {}", e);
        return ExitCode::from(5);
        }

    // Open output file
    let mut outptr = match File::create_new(output) {
        Ok(output_file) => output_file,
        Err(e) => {
            println!("Error creating new file: {}", e);
            return ExitCode::from(7);
        }
    };

    // Read infile's BITMAPFILEHEADER
    // Read the bytes into a buffer
    let mut fbuf = [0u8; mem::size_of::<BITMAPFILEHEADER>()];
    inptr.expect("REASON").read_exact(&mut fbuf);
    // Convert the buffer into the struct
    let bf = unsafe { *(fbuf.as_ptr() as *const BITMAPFILEHEADER) };

    // Read infile's BITMAPINFOHEADER
    // Read the bytes into a buffer
    let mut ibuf = [0u8; mem::size_of::<BITMAPINFOHEADER>()];
    inptr.expect("REASON").read_exact(&mut ibuf);
    // Convert the buffer into the struct
    let bi = unsafe { *(ibuf.as_ptr() as *const BITMAPINFOHEADER) };

    // Ensure infile is (likely) a 24-bit uncompressed BMP 4.0
    if bf.bfType != 0x4d42 || bf.bfOffBits != 54 || bi.biSize != 40 ||
        bi.biBitCount != 24 || bi.biCompression != 0 {
        println!("Unsupported file format.");
        return ExitCode::from(8);
    }

    // Get image's dimensions
    let height = bi.biHeight.abs();
    let width = bi.biWidth;

    // Allocate memory for image
    let image RGBTRIPLE(image)[width] = std::alloc(height, width * mem::size_of<RGBTRIPLE>);
    if image.is_null() {
        println!("Not enough memory to store image.");
        return ExitCode::from(9);
    }

    // Determine padding for scanlines
    let padding = (4 - (width * size_of<RGBTRIPLE>) % 4) % 4;

    // Iterate over infile's scanlines
    for i in 0..height {
        // Read row into pixel array
        // fread(image[i], sizeof(RGBTRIPLE), width, inptr);
        // Read the bytes in a buffer
        let mut scanbuf = [0u8; mem::size_of::<RGBTRIPLE>()];
        inptr.read_exact(&mut scanbuf);
        // Convert the buffer into a struct
        let image[i] = unsafe { *(scanbuf.as_ptr() as *const RGBTRIPLE) };

        // Skip over padding
        // fseek(inptr, padding, SEEK_CUR);
        inptr.seek(SeekFrom::Current(padding as i64));

        i += 1;
    }

    // Filter image
    // blur
    if matches.opt_present("b") {
        blur(height: i32, width: i32, image: RGBTRIPLE);
    }

    // Grayscale
    if matches.opt_present("g") {
        grayscale(height: i32, width: i32, image: RGBTRIPLE);
    }
    // Reflection
    if matches.opt_present("r") {
        reflect(height: i32, width: i32, image: RGBTRIPLE);
    }

    // Sepia
    if matches.opt_present("s") {
        sepia(height: i32, width: i32, image: RGBTRIPLE);
    }

    // Write outfile's BITMAPFILEHEADER
    std::io::write(outptr, mem::size_of::<BITMAPFILEHEADER>());

    // Write outfile's BITMAPINFOHEADER
    std::io::write(outptr, mem::size_of::<BITMAPINFOHEADER>());

    // Write new pixels to outfile
    for i in 0..height {
        // Write row to outfile
        std::io::write(outptr, image[i], mem::size_of::<RGBTRIPLE>);

        // Write padding at end of row
        for k in 0..padding {
            std::io::write(outptr, 0x00);

            k += 1;
        }

        i += 1;
    }

    // Free memory for image
    // This is not necessary as is handled by Rust

    // Close files
    // This is not necessary as is handled by Rust

    return 0;
}