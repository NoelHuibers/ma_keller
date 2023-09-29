mod remove_missings;
use crate::write_csv::write_csv;

use csv::ReaderBuilder;
use remove_missings::process_rows;
use std::collections::HashSet;
use std::error::Error;
use std::fs::File;
use std::vec::Vec;

pub fn remove_duplicate_restaurants_by_address_id(
    input_file_path: &str,
    output_file_path: &str,
) -> Result<(), Box<dyn Error>> {
    // Open the CSV file with a semicolon delimiter
    let file = File::open(input_file_path)?;
    let mut rdr = ReaderBuilder::new()
        .delimiter(b';') // Set the delimiter to semicolon
        .from_reader(file);

    // Create a HashSet to track unique AddressIDs
    let mut unique_address_ids: HashSet<String> = HashSet::new();

    // Create a vector to store rows to write to the output file
    let mut rows_to_write: Vec<Vec<String>> = Vec::new();

    // Iterate through the CSV records
    for result in rdr.records() {
        let record = result?;

        // Extract AddressID (assuming it's in the third column)
        if let Some(address_id) = record.get(2) {
            // Check if the AddressID is unique
            if !unique_address_ids.contains(address_id) {
                // If it's unique, add to the output
                rows_to_write.push(record.iter().map(|s| s.to_string()).collect());

                // Update the set
                unique_address_ids.insert(address_id.to_string());
            }
            // Else, check which one has the longer 3 record
            else {
                // Get the row that's already in the output
                let row_to_update = rows_to_write
                    .iter_mut()
                    .find(|row| row[2] == address_id)
                    .unwrap();

                // Get the row that's currently being processed
                let row_to_compare = record
                    .iter()
                    .map(|s| s.to_string())
                    .collect::<Vec<String>>();

                // If the row being processed has a longer 3 record, update the output
                if row_to_compare[3].len() > row_to_update[3].len() {
                    *row_to_update = row_to_compare;
                }
            }
        }
    }

    // Write the filtered rows to the output CSV file
    write_csv(output_file_path, &rows_to_write)?;

    let _ = process_rows(rows_to_write);

    Ok(())
}
