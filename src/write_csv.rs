use csv::WriterBuilder;
use std::error::Error;
use std::fs::File;
use std::vec::Vec;

pub fn write_csv(output_file_path: &str, rows: &[Vec<String>]) -> Result<(), Box<dyn Error>> {
    let file = File::create(output_file_path)?;
    let mut wtr = WriterBuilder::new()
        .delimiter(b';') // Set the delimiter to semicolon
        .from_writer(file);

    for row in rows {
        wtr.write_record(row)?;
    }

    wtr.flush()?;
    Ok(())
}
