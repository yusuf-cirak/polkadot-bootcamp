fn main() {
    // Create two strings with as String instances
    let string1 = String::from("Hello");
    let string2 = String::from(", world!");

    // Concanate the two strings using the function. Pass the string as references.
    let concatenated_string = concatenate_strings(&string1, &string2);

    // Print the result to console
    println!("{concatenated_string}");
}

// Week 1 Task
// Create a function that takes two string slice arguments and concatenates them into a String. Print the result to console.
fn concatenate_strings(string1: &str, string2: &str) -> String {
    // Create a new mutable string instance
    let mut result = String::new();

    // Push the two strings into the result string
    result.push_str(string1);
    result.push_str(string2);

    // Return the result
    result
}
