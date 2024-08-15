use std::io; // Import the library to handle user input

fn main() {
    // Display the options 
    println!("Temperature Converter");
    println!("Please select an option:");
    println!("1. Convert Celsius to Fahrenheit");
    println!("2. Convert Fahrenheit to Celsius");

    // Get the user's choice
    let choice = get_user_input("Enter your choice (1 or 2):");

    // Match the choice to perform the correct conversion
    match choice.trim() {
        "1" => {
            let celsius = get_temperature("Enter the temperature in Celsius: ");
            let fahrenheit = celsius_to_fahrenheit(celsius);
            println!("{}°C is equal to {}°F", celsius, fahrenheit);
        }
        "2" => {
            let fahrenheit = get_temperature("Enter the temperature in Fahrenheit: ");
            let celsius = fahrenheit_to_celsius(fahrenheit);
            println!("{}°F is equal to {}°C", fahrenheit, celsius);
        }
        _ => println!("Invalid choice! Please enter 1 or 2."),
    }
}

// Function to get user input and return it as a string
fn get_user_input(prompt: &str) -> String {
    println!("{}", prompt); // Display the prompt to the user
    let mut input = String::new(); // Create a new, empty string to store input
    io::stdin().read_line(&mut input).expect("Failed to read line"); // Read input from the user
    input // Return the input
}

// Function to get the temperature from the user and convert it to a floating-point number (f64)
fn get_temperature(prompt: &str) -> f64 {
    loop {
        let input = get_user_input(prompt); // Get the input as a string
        match input.trim().parse::<f64>() { // Try to parse the input into an f64 (floating-point number)
            Ok(num) => return num, // If successful, return the number
            Err(_) => println!("Invalid input, please enter a valid number."), // If there's an error, prompt again
        }
    }
}

// Function to convert Celsius to Fahrenheit
fn celsius_to_fahrenheit(celsius: f64) -> f64 {
    celsius * 9.0 / 5.0 + 32.0 // The formula for conversion
}

// Function to convert Fahrenheit to Celsius
fn fahrenheit_to_celsius(fahrenheit: f64) -> f64 {
    (fahrenheit - 32.0) * 5.0 / 9.0 // The formula for conversion
}
