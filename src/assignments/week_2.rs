use std::{io::stdin, str::FromStr};

// Week 2 Task
// Create a simple calculator using enums and pattern matching
enum Operation {
    Add { value1: f64, value2: f64 },
    Subtract { value1: f64, value2: f64 },
    Multiply { value1: f64, value2: f64 },
    Divide { value1: f64, value2: f64 },
}

fn calculate(operation: &Operation) -> f64 {
    match operation {
        Operation::Add { value1, value2 } => value1 + value2,
        Operation::Subtract { value1, value2 } => value1 - value2,
        Operation::Multiply { value1, value2 } => value1 * value2,
        Operation::Divide { value1, value2 } => value1 / value2,
    }
}

pub fn execute_calculate() {
    // Week 2 Task
    // Create a simple calculator using enums and pattern matching

    // Create a constant variable for the fail message
    const FAIL_MESSAGE: &str = "Failed to read line";

    // Create a new mutable string instances for reading inputs from user
    let mut operation = String::new();
    let mut value1 = String::new();
    let mut value2 = String::new();

    // Read the inputs from user and assign them into the mutable string instances
    println!("Please provide an operation by using symbols: ");
    stdin().read_line(&mut operation).expect(FAIL_MESSAGE);

    println!("Please provide the first value: ");
    stdin().read_line(&mut value1).expect(FAIL_MESSAGE);

    println!("Please provide the second value: ");
    stdin().read_line(&mut value2).expect(FAIL_MESSAGE);

    // Parse the values into f64
    // Note: We're shadowing the previous values but its okay since we're not using them anymore
    let value1 = f64::from_str(value1.trim()).expect(FAIL_MESSAGE);
    let value2 = f64::from_str(value2.trim()).expect(FAIL_MESSAGE);

    // Create the operation instance by using pattern matching and parsing the values into f64

    let operation = match operation.trim() {
        "+" => Operation::Add { value1, value2 },
        "-" => Operation::Subtract { value1, value2 },
        "*" => Operation::Multiply { value1, value2 },
        "/" => Operation::Divide { value1, value2 },
        _ => panic!("Invalid operation"), // Panic if the operation is not valid so the program will not continue
    };

    // Get the result by using the calculate function
    let result = calculate(&operation);

    // Print the result to console
    println!("The result is: {}", result);
}
