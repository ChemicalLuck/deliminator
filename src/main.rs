mod cli;
mod paths;
mod records;

use clap::Parser;
use csv::StringRecord;
use rayon::prelude::*;

use cli::{Cli, Commands};
use paths::{create_output_directory, determine_output_path, generate_output_path};
use records::{load_records, write_records_to_csv};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let cli = Cli::parse();
    let start = std::time::Instant::now();

    let mut records = load_records(&cli)?;

    let header = records.remove(0);
    let num_records = records.len();

    let records_per_file = match &cli.command {
        Commands::Chunks(args) => args.chunks,
        Commands::Files(args) => num_records / args.files,
    };

    let output = determine_output_path(&cli, &records_per_file)?;

    create_output_directory(&output)?;

    let chunks: Vec<&[StringRecord]> = records.chunks(records_per_file).collect();

    let progress_bar = indicatif::ProgressBar::new(chunks.len() as u64);

    chunks.par_iter().enumerate().for_each(|(i, chunk)| {
        let output_path = generate_output_path(&output, &cli.csv_path, i).unwrap();
        write_records_to_csv(&header, chunk, &output_path).unwrap();
        progress_bar.inc(1);
    });

    progress_bar.finish();

    let duration = start.elapsed();
    println!("Results written to: {}", output.display());
    println!("Completed in {} milliseconds", duration.as_millis());

    Ok(())
}
