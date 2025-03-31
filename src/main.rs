use std::env;
use std::fs::File;
use std::io::{self};

macro_rules! debug_println {
    ($($arg:tt)*) => {
        #[cfg(debug_assertions)]
        println!($($arg)*)
    }
}

fn main() {
    // Get command-line arguments
    let args: Vec<String> = env::args().collect();
    debug_println!("\narguments: {:?}\n", args);

    // If no files were provided, read from stdin
    if args.len() < 2 {
        if let Err(err) = cat_stdin() {
            eprintln!("Error reading from stdin: {}", err);
        }
    } else {
        // Process each file
        for filename in &args[1..] {
            if let Err(err) = cat_file(filename) {
                eprintln!("Error processing {}: {}", filename, err);
            }
        }
    }
}

// Function to display contents of a file
fn cat_file(filename: &str) -> io::Result<()> {
    // Open the file
    let mut file = File::open(filename)?;

    // Copy all bytes from the file to stdout
    io::copy(&mut file, &mut io::stdout())?;

    Ok(())
}

// Function to display contents from stdin
fn cat_stdin() -> io::Result<()> {
    // Copy all bytes from stdin to stdout
    io::copy(&mut io::stdin(), &mut io::stdout())?;

    Ok(())
}
