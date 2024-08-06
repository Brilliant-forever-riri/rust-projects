use std::io;

fn main() {
    loop {
        println!("Simple Calculator");
        println!("Please select an operation:");
        println!("1. Addition");
        println!("2. Subtraction");
        println!("3. Multiplication");
        println!("4. Division");
        println!("5. Modulus");
        println!("6. Exit");

        let choice = get_user_input("Enter your choice: ");

        match choice.trim() {
            "1" => perform_operation(addition),
            "2" => perform_operation(subtraction),
            "3" => perform_operation(multiplication),
            "4" => perform_operation(division),
            "5" => perform_operation(modulus),
            "6" => {
                println!("Exiting...");
                break;
            }
            _ => println!("Invalid choice, please try again."),
        }
    }
}

fn get_user_input(prompt: &str) -> String {
    println!("{}", prompt);
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read line");
    input
}

fn perform_operation(operation: fn(f64, f64) -> f64) {
    let num1 = get_number("Enter the first number: ");
    let num2 = get_number("Enter the second number: ");
    let result = operation(num1, num2);
    println!("The result is: {}", result);
}

fn get_number(prompt: &str) -> f64 {
    loop {
        let input = get_user_input(prompt);
        match input.trim().parse::<f64>() {
            Ok(num) => return num,
            Err(_) => println!("Invalid input, please enter a valid number."),
        }
    }
}

fn addition(a: f64, b: f64) -> f64 {
    a + b
}

fn subtraction(a: f64, b: f64) -> f64 {
    a - b
}

fn multiplication(a: f64, b: f64) -> f64 {
    a * b
}

fn division(a: f64, b: f64) -> f64 {
    if b == 0.0 {
        println!("Error: Division by zero is not allowed.");
        return 0.0;
    }
    a / b
}

fn modulus(a: f64, b: f64) -> f64 {
    if b == 0.0 {
        println!("Error: Division by zero is not allowed.");
        return 0.0;
    }
    a % b
}

