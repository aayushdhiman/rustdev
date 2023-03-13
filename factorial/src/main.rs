// using standard io
use std::io;

// factorial function (recursive)
fn factorial(num: u64) -> u64 {
    if num <= 1 {
        return 1;
    }

    num * factorial(num - 1)
}

// input handling, parsing, etc.
fn main() {
    println!("What should I find the factorial of? ");

    // create (new) mutable string
    let mut input = String::new();
    // using io, read the line and if it fails, print an error message
    io::stdin().read_line(&mut input).expect("Failed to read input.");
    // create a new number that is the input, trimmed, parsed to be an unsigned 64 bit integer
    // if it fails, print an error message
    let num = input.trim().parse::<u64>().expect("Failed to parse integer.");
    
    println!("{}", factorial(num));   
}
