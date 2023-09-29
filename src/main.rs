//mod find_duplicates;
//mod write_csv;
mod find_missings;
mod remove_duplicates;
mod write_csv;
//use find_duplicates::find_duplicate_restaurants;
//use write_csv::write_duplicate_csvs;
use find_missings::filter_rows_by_empty_column;
use remove_duplicates::remove_duplicate_restaurants_by_address_id;
use std::error::Error;

fn main() -> Result<(), Box<dyn Error>> {
    // Specify the path to your CSV file
    let input_file_path_restaurns: &str = "input/Restaurant.csv";
    let input_file_path_sales: &str = "input/Sales.csv";

    //let (name_duplicate_rows, address_id_duplicate_rows, address_combination_duplicate_rows) =
    //    find_duplicate_restaurants(input_file_path)?;

    // Specify the paths for the output CSV files
    //let name_duplicates_file = "output/name_duplicates.csv";
    //let address_id_duplicates_file = "output/address_id_duplicates.csv";
    //let address_combination_duplicates_file = "output/address_combination_duplicates.csv";

    // Call the write_duplicate_csvs function to write the duplicate rows to separate CSV files
    //write_duplicate_csvs(
    //    name_duplicates_file,
    //    address_id_duplicates_file,
    //    address_combination_duplicates_file,
    //    &name_duplicate_rows,
    //    &address_id_duplicate_rows,
    //    &address_combination_duplicate_rows,
    //)?;

    remove_duplicate_restaurants_by_address_id(
        input_file_path_restaurns,
        "output/Restaurant_no_duplicates.csv",
    )?;

    filter_rows_by_empty_column(
        input_file_path_restaurns,
        "output/Restaurant_no_alcohol.csv",
        8,
    )?;

    filter_rows_by_empty_column(input_file_path_sales, "output/Sales_no_overall.csv", 16)?;

    Ok(())
}
