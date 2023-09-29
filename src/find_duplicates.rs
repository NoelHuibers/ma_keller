use csv::ReaderBuilder;
use std::collections::{HashMap, VecDeque};
use std::error::Error;
use std::fs::File;
use std::vec::Vec;

#[derive(Debug, Eq, PartialEq, Hash)]
struct Address {
    street: String,
    city: String,
    state: String,
    country: String,
    zip: String,
}

pub fn find_duplicate_restaurants(
    input_file_path: &str,
) -> Result<(Vec<Vec<String>>, Vec<Vec<String>>, Vec<Vec<String>>), Box<dyn Error>> {
    // Open the CSV file with a semicolon delimiter
    let file = File::open(input_file_path)?;
    let mut rdr = ReaderBuilder::new()
        .delimiter(b';') // Set the delimiter to semicolon
        .from_reader(file);

    // Create a HashMap to store restaurant names and their data
    let mut restaurant_map: HashMap<String, Vec<Vec<String>>> = HashMap::new();

    // Create a HashMap to store duplicate AddressIDs
    let mut address_id_map: HashMap<String, Vec<Vec<String>>> = HashMap::new();

    // Create a HashMap to store duplicate Address combinations
    let mut address_combination_map: HashMap<Address, Vec<Vec<String>>> = HashMap::new();

    // Create a queue to store names with duplicates
    let mut name_queue: VecDeque<String> = VecDeque::new();

    // Iterate through the CSV records
    for result in rdr.records() {
        let record = result?;

        // Extract the restaurant name (assuming it's in the second column)
        if let Some(name) = record.get(1) {
            // Check if we already have a restaurant with the same name
            // If yes, append the current data to the existing data
            if let Some(existing_data) = restaurant_map.get_mut(name) {
                existing_data.push(record.iter().map(|s| s.to_string()).collect());
                name_queue.push_back(name.to_string());
            } else {
                // If no restaurant with the same name exists, create a new entry
                let data: Vec<Vec<String>> = vec![record.iter().map(|s| s.to_string()).collect()];
                restaurant_map.insert(name.to_string(), data);
            }
        }

        // Extract AddressID (assuming it's in the third column)
        if let Some(address_id) = record.get(2) {
            // Check if AddressID already exists
            if let Some(existing_data) = address_id_map.get_mut(address_id) {
                existing_data.push(record.iter().map(|s| s.to_string()).collect());
            } else {
                // If no restaurant with the same AddressID exists, create a new entry
                let data: Vec<Vec<String>> = vec![record.iter().map(|s| s.to_string()).collect()];
                address_id_map.insert(address_id.to_string(), data);
            }
        }

        // Extract Address components
        let address = Address {
            street: record[3].to_string(),
            city: record[4].to_string(),
            state: record[5].to_string(),
            country: record[6].to_string(),
            zip: record[7].to_string(),
        };

        // Check if Address combination already exists
        if let Some(existing_data) = address_combination_map.get_mut(&address) {
            existing_data.push(record.iter().map(|s| s.to_string()).collect());
        } else {
            // If no restaurant with the same Address combination exists, create a new entry
            let data: Vec<Vec<String>> = vec![record.iter().map(|s| s.to_string()).collect()];
            address_combination_map.insert(address, data);
        }
    }

    // Collect the duplicate rows into separate vectors
    let mut duplicate_name_rows: Vec<Vec<String>> = Vec::new();
    let mut duplicate_address_id_rows: Vec<Vec<String>> = Vec::new();
    let mut duplicate_address_combination_rows: Vec<Vec<String>> = Vec::new();

    while let Some(name) = name_queue.pop_front() {
        if let Some(data) = restaurant_map.get(&name) {
            if data.len() > 1 {
                duplicate_name_rows.extend_from_slice(&data[1..]);
            }
        }
    }

    for (_, data) in address_id_map.iter() {
        if data.len() > 1 {
            duplicate_address_id_rows.extend_from_slice(&data[1..]);
        }
    }

    for (_, data) in address_combination_map.iter() {
        if data.len() > 1 {
            duplicate_address_combination_rows.extend_from_slice(&data[1..]);
        }
    }

    Ok((
        duplicate_name_rows,
        duplicate_address_id_rows,
        duplicate_address_combination_rows,
    ))
}
