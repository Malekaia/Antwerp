use glob::glob;
use std::fs;

pub fn read_file(file_path: &str) -> String {
  match fs::read_to_string(file_path) {
    Ok(file_content) => file_content,
    Err(_) => panic!("ReadError: failed to read \"{file_path}\"")
  }
}

#[allow(unused)]
pub fn write_file(file_path: &str, contents: String) {
  fs::write(file_path, contents).expect("WriteError: failed to write to file");
}

pub fn walk(path: &str) -> impl Iterator<Item = String> {
  glob(path)
    .expect("GlobError: Failed to read glob pattern")
    .map(| entry |
      entry.expect("GlobError: failed to glob entry").display().to_string()
    )
}
