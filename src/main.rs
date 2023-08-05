use assignments::week_3::*;

mod assignments;
fn main() {
    // Week 1 Task
    // Create a function that takes two string slice arguments and concatenates them into a String. Print the result to console.
    // execute_concanate_strings();
    // assignments::week_1::execute_concanate_strings();

    // Week 2 Task
    // Create a simple calculator using enums and pattern matching
    // assignments::week_2::execute_calculate();

    // Week 3 Task
    // Create a custom filtering function
    // Please inspect my code in src\assignments\week_3.rs
    let collection = vec![1, 2, 3, 4, 5];
    let filter_condition: FilterCondition<i32> = FilterCondition::new(|&item| item > 1);

    let filtered_collection = custom_filter(&collection, &filter_condition);

    println!("{:?}", filtered_collection);
}
