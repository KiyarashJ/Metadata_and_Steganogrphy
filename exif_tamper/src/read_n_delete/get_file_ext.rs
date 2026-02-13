use std::path::Path;

pub fn get_ext(file_path: &Path) -> Option<&str> {
    file_path.extension().and_then(|os| os.to_str())
}
