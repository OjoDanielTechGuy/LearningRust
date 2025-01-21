mod review;

use std::io; // standard library for standard input and output

fn main() {
    //main function
    println!("Guess the number."); //rust macro to print out

    println!("Input the number: ");

    /*
     * mut - makes the variable mutable as all variables are immutable in rust
     * = String::new() - indicate a string value to be assigned to the variable and new() to hold the variable
     */

    let mut guess = String::new();

    /*
    io::stdin() - we call stdin() function from teh module io to handle input from the system
    .read_line(&mut variable_name) - reads the variable by passing it the arguement of &mut and variable name
    .expect("..") - process teh error part if it does not read the arguement
     */
    io::stdin() //
        .read_line(&mut guess)
        .expect("Failed to read line");
    println!("You guessed: {guess}");
}
