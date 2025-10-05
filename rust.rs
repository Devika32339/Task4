// Import the necessary modules from the chrono crate
use chrono::{Local, Timelike};

fn main() {
    // Get the current local date and time
    let now = Local::now();

    // Get the name from the environment or use a default
    let name = "Devika"; // You can change this to your name

    // Format the time as a string
    let formatted_time = now.format("%Y-%m-%d %H:%M:%S").to_string();

    // Print the greeting with the name and formatted time
    println!("Hello {}, right now the time is {}", name, formatted_time);
}
