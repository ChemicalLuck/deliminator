use std::{
    ffi::OsStr,
    fs,
    path::{Path, PathBuf},
};

pub fn determine_output_path(
    output: &Option<PathBuf>,
    path: &OsStr,
    command: &str,
    records_per_file: &usize,
) -> Result<PathBuf, Box<dyn std::error::Error>> {
    let default_output_path = dirs::home_dir()
        .map(|p| p.join("Documents/csv-split"))
        .unwrap_or_else(|| PathBuf::from("~"));

    let output_path = output
        .clone()
        .unwrap_or_else(|| default_output_path)
        .join(format!(
            "{}_{}_{}/",
            path.to_str().unwrap(),
            command,
            records_per_file
        ));

    Ok(output_path)
}

pub fn create_output_directory(output: &Path) -> Result<(), Box<dyn std::error::Error>> {
    if !output.exists() {
        fs::create_dir_all(output)?;
    }
    Ok(())
}

pub fn generate_output_path(
    output: &Path,
    csv_path: &Path,
    index: usize,
) -> Result<PathBuf, Box<dyn std::error::Error>> {
    let output_name = format!(
        "{}_part{}.csv",
        csv_path.file_stem().unwrap().to_str().unwrap(),
        index
    );

    let mut output_path = output.to_path_buf();
    if output.is_dir() {
        output_path = output_path.join(&output_name);
    } else {
        output_path.set_file_name(&output_name);
    }

    Ok(output_path)
}
