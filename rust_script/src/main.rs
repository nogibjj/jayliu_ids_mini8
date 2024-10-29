use polars::prelude::*;
use std::fs::File;
use std::io::BufReader;
use std::error::Error;

fn process_data(file_path: &str) -> std::result::Result<(), Box<dyn Error>> {
    // Open the file and wrap it in a BufReader for Polars to read
    let file = File::open(file_path)?;
    let reader = BufReader::new(file);
    
    // Load the CSV into a DataFrame
    let df = CsvReader::new(reader).finish()?;

    // Filter rows where 'year' is greater than 2000
    let filtered_df = df.filter(&df.column("year")?.gt(2000)?)?;
    
    // Group by 'month' and aggregate birth counts
    let mut summary_by_month = filtered_df.groupby(["month"])?.agg(&[("births", &["sum"])])?;
    let mut summary_by_day = filtered_df.groupby(["day_of_week"])?.agg(&[("births", &["sum"])])?;

    // Ensure the output directory exists
    std::fs::create_dir_all("data")?;

    // Write results to CSV files
    CsvWriter::new(File::create("data/birth_summary_by_month.csv")?).finish(&mut summary_by_month)?;
    CsvWriter::new(File::create("data/birth_summary_by_day.csv")?).finish(&mut summary_by_day)?;

    Ok(())
}

fn main() {
    // Define the relative path to the CSV file
    let file_path = "../data/US_birth.csv";

    // Run the data processing function
    if let Err(e) = process_data(file_path) {
        eprintln!("Error processing data: {:?}", e);
    }
}


