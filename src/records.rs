use std::path::Path;

use csv::{ReaderBuilder, StringRecord, WriterBuilder};

pub fn load_records(
    path: &Path,
) -> Result<Vec<StringRecord>, Box<dyn std::error::Error + 'static>> {
    Ok(ReaderBuilder::new()
        .has_headers(false)
        .from_path(path)?
        .records()
        .collect::<Result<Vec<_>, _>>()?)
}

pub fn write_records_to_csv(
    header: &StringRecord,
    chunk: &[StringRecord],
    output_path: &Path,
) -> Result<(), Box<dyn std::error::Error>> {
    let mut csv_writer = WriterBuilder::new()
        .has_headers(true)
        .from_path(output_path)?;

    csv_writer.write_record(header)?;

    for record in chunk {
        csv_writer.write_record(record)?;
    }

    csv_writer.flush()?;
    Ok(())
}
