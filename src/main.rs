use glob::glob;
use marcus;
use std::fs;

fn read_file(file_path: &str) -> String {
  match fs::read_to_string(file_path) {
    Ok(file_content) => file_content,
    Err(_) => panic!("ReadError: failed to read \"{file_path}\"")
  }
}

fn write_file(file_path: &str, contents: String) {
  fs::write(file_path, contents).expect("WriteError: failed to write to file");
}

fn walk(path: &str) -> impl Iterator<Item = String> {
  glob(path)
    .expect("GlobError: Failed to read glob pattern")
    .map(| entry |
      entry.expect("GlobError: failed to glob entry").display().to_string()
    )
}

fn main() -> () {
  // Glob the MD test files
  for file_path in walk("./test/**/*.md") {

  }
}
