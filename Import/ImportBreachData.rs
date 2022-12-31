extern crate sled;

use std::io;
use std::fs;
use std::path::Path;

fn main() {
    // Create a new Sled database
    let db = sled::open("breach_data.db").unwrap();

    println!("Enter the path to the file or files to import (separated by a space):");

    // Read the user input
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    
    println!("What breach or site did this data come from?",);

    // Read the breach location from the user
    let mut breach_location = String::new();
    io::stdin().read_line(&mut breach_location).unwrap();
    let breach_location = breach_location.trim();

    // Split the input on whitespace to get the list of file paths
    let file_paths: Vec<&str> = input.split_whitespace().collect();

    // Iterate over the file paths
    for file_path in file_paths {
        // Open the file
        let file = fs::File::open(file_path).unwrap();
        let reader = io::BufReader::new(file);

        // Iterate over the lines in the file
        for line in reader.lines() {
            let line = line.unwrap();

            // Split the line at the `:` character
            let parts: Vec<&str> = line.split(':').collect();
            let email = parts[0];
            let password = parts[1];

            // Insert the email, password, and third variable into the database as a tuple
            db.insert(email, (password, breach_location)).unwrap();
        }
    }
}
