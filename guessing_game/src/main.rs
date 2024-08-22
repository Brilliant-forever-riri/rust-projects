use std::io;      // Import the input/output library to handle user input
use rand::Rng;    // Import the random number generator from the rand crate
use std::cmp::Ordering;  // Import the Ordering enum to compare the guess with the secret number

fn main() {
    println!("Welcome to the Guessing Game!");

    // Generate a random number between 1 and 100
    let secret_number = rand::thread_rng().gen_range(1..=100);

    loop {
        println!("Please input your guess:");

        // Get user input and store it in a mutable variable
        let mut guess = String::new();
        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line");

        // Convert the input string to a number
        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,        // If parsing succeeds, use the number
            Err(_) => {
                println!("Please enter a valid number.");
                continue;           // Skip to the next loop iteration if parsing fails
            }
        };

        // Compare the guess with the secret number
        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Too small!"),   // If the guess is too small
            Ordering::Greater => println!("Too big!"),  // If the guess is too big
            Ordering::Equal => {
                println!("You win! The number {secret_number}");                   // If the guess is correct
                break;                                  // Exit the loop
            },
            _ => println!("game over") //this takes in a default
        }
    }
}
