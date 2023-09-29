use crate::write_csv::write_csv;

use csv::WriterBuilder;
use std::error::Error;
use std::fs::File;

pub fn write_duplicate_csvs(
    name_duplicates: &str,
    address_id_duplicates: &str,
    address_combination_duplicates: &str,
    name_duplicate_rows: &[Vec<String>],
    address_id_duplicate_rows: &[Vec<String>],
    address_combination_duplicate_rows: &[Vec<String>],
) -> Result<(), Box<dyn Error>> {
    // Write duplicate name rows to a CSV file
    write_csv(name_duplicates, name_duplicate_rows)?;

    // Write duplicate AddressID rows to a CSV file
    write_csv(address_id_duplicates, address_id_duplicate_rows)?;

    // Write duplicate Address combination rows to a CSV file
    write_csv(
        address_combination_duplicates,
        address_combination_duplicate_rows,
    )?;

    Ok(())
}
