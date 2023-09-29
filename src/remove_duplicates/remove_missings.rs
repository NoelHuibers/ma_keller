use ndarray::{Array2, Axis};
use std::error::Error;

use crate::write_csv::write_csv;

pub fn process_rows(rows: Vec<Vec<String>>) -> Result<(), Box<dyn Error>> {
    // Create a vector to store rows with a valid entry in the "Alcohol" column
    let mut filtered_rows: Vec<Vec<String>> = Vec::new();

    // Iterate through the rows and retain rows with a non-empty "Alcohol" column
    for row in rows {
        // Assuming "Alcohol" column is in index 8 (zero-based index)
        if let Some(alcohol_value) = row.get(8) {
            if !alcohol_value.trim().is_empty() {
                // Add the row to the filtered_rows vector
                filtered_rows.push(row);
            }
        }
    }

    // Convert the data to a numeric array (Array2<f64>)
    let numeric_data: ndarray::ArrayBase<ndarray::OwnedRepr<f64>, ndarray::Dim<[usize; 2]>> =
        convert_to_numeric_array(&filtered_rows)?;

    println!("Correlation Matrix:\n{:?}", numeric_data);

    // Write the filtered rows to a new output CSV file
    let output_file_path = "output/output_filtered.csv"; // Change the output file name as needed
    write_csv(output_file_path, &filtered_rows)?;

    Ok(())
}

fn convert_to_numeric_array(data: &Vec<Vec<String>>) -> Result<Array2<f64>, Box<dyn Error>> {
    let rows = data.len();
    let cols = data[0].len();

    let mut numeric_data = Array2::zeros((rows, cols));

    for (i, row) in data.iter().enumerate() {
        for (j, val) in row.iter().enumerate() {
            let parsed_value = val.parse::<f64>()?;
            numeric_data[[i, j]] = parsed_value;
        }
    }

    Ok(numeric_data)
}
