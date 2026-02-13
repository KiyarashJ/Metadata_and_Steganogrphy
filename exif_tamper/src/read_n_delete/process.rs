
use metastrip::Metadata;
use crate::read_n_delete::{get_file_ext::{get_ext}, random::random};
use std::{fs::File, io::{BufWriter, Write}};

pub enum Typefile {
    No(&'static str),
    Buffer(std::path::PathBuf)
}

pub enum Typeexif {
    No(&'static str),
    Type(Metadata)
}


pub fn process(file_name: &str, exif: Typeexif, file_path: Typefile, copy: i8) -> Result<&str, Box<dyn std::error::Error>> {
    let rnd = random();
    let _ = std::fs::create_dir_all("metadatas"); 
    let file_name_ = file_name.split('.').next().unwrap_or(file_name);
    match exif {
        Typeexif::Type(exif_data) => {
            let filename = format!("output-{rnd:?}-{file_name}.txt");
            let full_path = std::path::Path::new("metadatas").join(filename);
            let file = File::create(&full_path)?;
            let mut buffer = BufWriter::new(file);
            writeln!(buffer, "{:#?}", exif_data)?; 
            buffer.flush()?;  
        }

        Typeexif::No(_) => {
            if copy == 1 {
                if let Typefile::Buffer(path) = file_path {
                    let ext = get_ext(&path).unwrap();
                    let full_dest = format!("metadatas/copied-{rnd:?}-{file_name_}.{ext}");
                    std::fs::copy(path, full_dest)?;
                }
            }
        }
    }

    Ok("Done!")
}
