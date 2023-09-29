use crate::write_csv::write_csv;

use csv::ReaderBuilder;
use std::error::Error;
use std::fs::File;
use std::vec::Vec;

pub fn filter_rows_by_empty_column(
    input_file_path: &str,
    output_file_path: &str,
    column_index: usize,
) -> Result<(), Box<dyn Error>> {
    // Open the CSV file with a semicolon delimiter
    let file = File::open(input_file_path)?;
    let mut rdr = ReaderBuilder::new()
        .delimiter(b';') // Set the delimiter to semicolon
        .from_reader(file);

    // Create a vector to store rows with an empty or missing column entry
    let mut filtered_rows: Vec<Vec<String>> = Vec::new();

    // Iterate through the CSV records
    for result in rdr.records() {
        let record = result?;

        // Check if the specified column index is valid
        if let Some(column_value) = record.get(column_index) {
            // Apply the condition: If the column value is empty or missing, add the row to the output
            if column_value.trim().is_empty() {
                filtered_rows.push(record.iter().map(|s| s.to_string()).collect());
            }
        } else {
            // If the column doesn't exist in the record, add the row to the output
            filtered_rows.push(record.iter().map(|s| s.to_string()).collect());
        }
    }

    // Write the filtered rows to the output CSV file
    write_csv(output_file_path, &filtered_rows)?;

    Ok(())
}
