extern crate getopts;
use getopts::Options;
use hanoi::{hanoi, hanoi_steps, TOWERS};
use std::env::args;

// Prints a help message for the command-line interface of the program.
fn program_description(program: &str, options: Options) {
    let usage: String = format!(
        "usage: {} [OPTIONS] N\n\nPrint steps needed to solve a Towers of Hanoi with N disks.\n{}",
        program, TOWERS
    );
    print!("{}", options.usage(&usage));
}

fn main() {
    // Collect command-line arguments.
    let args: Vec<String> = args().collect();
    let program: String = args[0].clone();

    // Declare options.
    let mut options = Options::new();
    options.optflag("c", "count", "print the number of steps needed to solve a tower with n disks instead of steps themselves.");
    options.optflag("n", "no-visualization", "do not print Tower of Hanoi ascii art at the top of output.");
    options.optflag("h", "help", "prints this help message.");

    // Match on options.
    let matches = match options.parse(&args[1..]) {
        Ok(m) => m,
        Err(e) => {
            panic!("{}", e.to_string())
        }
    };

    // Print help message if "h" option was used and exit.
    if matches.opt_present("h") {
        program_description(&program, options);
        return;
    }
    // Check whether "c" option was used.
    let count: bool = matches.opt_present("c");
    let quiet: bool = matches.opt_present("n");

    // Extract positional argument `N`
    let disks: u32 = if !matches.free.is_empty() {
        match matches.free[0].clone().parse() {
            Ok(n) => n,
            // Return error if unable to parse an argument into unsigned integer.
            Err(_) => panic!("Number of disks (N) must be a number!"),
        }
    } else {
        // Print a help message if no positional arguments were passed.
        program_description(&program, options);
        return;
    };

    // If "c" option was used print a number of steps and exit.
    if count {
        println!("{}", hanoi_steps(disks));
        return;
    }
    // If no options were used, calculate the steps and then print them to stdout.
    if !quiet {
        println!("{}", TOWERS);
    }
    hanoi(disks, 1, 3);
}
