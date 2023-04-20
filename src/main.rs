use csv::{ReaderBuilder, WriterBuilder};
use std::{error::Error, path::PathBuf};

fn parse_args() -> Result<(PathBuf, usize), Box<dyn Error>> {
    let args: Vec<String> = std::env::args().collect();

    if args.len() != 3 {
        return Err("Usage: split_csv <csv_path> <num_files>".into());
    }

    let csv_path = PathBuf::from(&args[1]);
    let num_files = args[2].parse::<usize>()?;

    Ok((csv_path, num_files))
}

fn main() -> Result<(), Box<dyn Error>> {
    let args = parse_args()?;

    let start = std::time::Instant::now();

    let csv_path = args.0;
    let num_files = args.1;

    println!("Spliting {} into {} files", csv_path.display(), num_files);

    let mut records = ReaderBuilder::new()
        .has_headers(false)
        .from_path(&csv_path)?
        .records()
        .collect::<Result<Vec<_>, _>>()?;

    let header = records.remove(0);
    let num_records = records.len();

    println!("Number of records: {}", num_records);

    let records_per_file = num_records / num_files;

    println!("Records per file: {}", records_per_file);

    let chunks = records.chunks(records_per_file);

    // Refactor this to a function
    for (i, chunk) in chunks.enumerate() {
        let output_name = format!(
            "{}_part{}.csv",
            csv_path.file_stem().unwrap().to_str().unwrap(),
            i
        );

        let output_path = csv_path.with_file_name(&output_name);

        let mut csv_writer = WriterBuilder::new()
            .has_headers(true)
            .from_path(output_path)?;

        csv_writer.write_record(&header)?;
        for record in chunk {
            csv_writer.write_record(record)?;
        }
        println!("Wrote {} records to {}", chunk.len(), output_name);
    }

    let duration = start.elapsed();
    println!("Completed in {} seconds", duration.as_secs());

    Ok(())
}
