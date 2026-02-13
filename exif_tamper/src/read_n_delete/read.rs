use std::{fs, path::PathBuf};
use crate::read_n_delete::{process::{self, process}};

pub fn read(
    file_path: &PathBuf,
    file_name: &str,
) -> Result<(), Box<dyn std::error::Error + Send + Sync + 'static>> {
    let data = fs::read(file_path)?;
    let extract = metastrip::extract_metadata(&data)?;
    let _ = process(file_name, 
                process::Typeexif::Type(extract), 
                process::Typefile::No("no data")
                , 0
    );
    Ok(())
}