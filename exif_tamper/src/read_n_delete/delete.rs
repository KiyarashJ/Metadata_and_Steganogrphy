use std::process::Command;
use std::path::{PathBuf};
use crate::read_n_delete::process::{self, Typeexif, process};

pub fn delete(
    file_path: &PathBuf,
    file_name: &str,
    copy: i8,
) -> Result<(), Box<dyn std::error::Error + Send + Sync + 'static>> {
    let path = file_path.as_path();
    let _ = process(
        file_name,
        Typeexif::No(""),
        process::Typefile::Buffer(file_path.clone()),
        copy
    );
    let output = Command::new("exiftool")
        .arg("-overwrite_original")   
        .arg("-all=")                
        .arg(path)
        .output()?;

    if !output.status.success() {
        let err = String::from_utf8_lossy(&output.stderr);
        return Err(format!("exiftool failed: {}", err).into());
    }

    Ok(())
}