mod cli;
mod paths;
mod records;

use std::path::PathBuf;

use clap::Parser;
use csv::StringRecord;
use rayon::prelude::*;

use cli::{Cli, Commands};
use paths::{create_output_directory, determine_output_path, generate_output_path};
use records::{load_records, write_records_to_csv};

fn split_into_chunks(
    cli: &Cli,
    mut records: Vec<StringRecord>,
) -> Result<(), Box<dyn std::error::Error>> {
    let num_records = records.len();
    let records_per_file = match &cli.command {
        Commands::Chunks(args) => args.chunks,
        Commands::Files(args) => num_records / args.files,
        _ => panic!("Invalid command"),
    };

    let header = records.remove(0);
    let output = determine_output_path(
        &cli.output,
        cli.csv_path.file_stem().unwrap(),
        &cli.command.name(),
        &records_per_file,
    )?;

    create_output_directory(&output)?;

    let chunks: Vec<&[StringRecord]> = records.chunks(records_per_file).collect();

    let progress_bar = indicatif::ProgressBar::new(chunks.len() as u64);

    chunks.par_iter().enumerate().for_each(|(i, chunk)| {
        let output_path = generate_output_path(&output, &cli.csv_path, i).unwrap();
        write_records_to_csv(&header, chunk, &output_path).unwrap();
        progress_bar.inc(1);
    });

    progress_bar.finish();
    println!("Results written to: {}", output.display());

    Ok(())
}

fn concat(
    cli: &Cli,
    others: &Vec<PathBuf>,
    mut records: Vec<StringRecord>,
) -> Result<(), Box<dyn std::error::Error>> {
    let output = determine_output_path(
        &cli.output,
        cli.csv_path.file_stem().unwrap(),
        &cli.command.name(),
        &records.len(),
    )?;

    println!("{}", output.display());

    create_output_directory(&output)?;
    let output_path = generate_output_path(&output, &cli.csv_path, 0).unwrap();

    let header = records.remove(0);

    for path in others {
        let mut other_records = load_records(&path)?;
        other_records.remove(0);
        records.append(&mut other_records);
    }

    write_records_to_csv(&header, &records, &output_path)?;
    println!("Results written to: {}", output.display());

    Ok(())
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let cli = Cli::parse();
    let start = std::time::Instant::now();

    let records = load_records(&cli.csv_path)?;

    match &cli.command {
        Commands::Chunks(_) => split_into_chunks(&cli, records)?,
        Commands::Files(_) => split_into_chunks(&cli, records)?,
        Commands::Concat(args) => concat(&cli, &args.other, records)?,
    }

    let duration = start.elapsed();
    println!("Completed in {} milliseconds", duration.as_millis());

    Ok(())
}
