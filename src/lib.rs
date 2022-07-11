pub mod file_system {
  use std::{fs, fs::File, path::Path, io::prelude::Write};

  pub fn read_file(file_path: &str) -> String {
    fs::read_to_string(file_path).expect(&format!("Error: failed to read file: \"{}\"", file_path))
  }

  // https://doc.rust-lang.org/std/fs/struct.File.html
  pub fn write_file(path: &str, content: &String) {
    // ensure the directory
    let mut dir: Vec<&str> = path.split("/").collect::<Vec<&str>>();
    dir.pop();
    super::file_system::ensure_dir(&dir.join("/"));
    // write the file
    let error_message: &str = &format!("Error: failed to write to \"{}\"", &path);
    let mut file: File = File::create(path).expect(error_message);
    file.write_all(content.as_bytes()).expect(error_message);
  }

  // https://doc.rust-lang.org/std/fs/fn.copy.html
  pub fn copy_file(from: &str, to: &str, overwrite: bool) {
    let mut to_path: Vec<&str> = to.split("/").collect::<Vec<&str>>();
    to_path.pop();
    super::file_system::ensure_dir(&to_path.join("/"));
    if overwrite == true || !super::file_system::exists(to) {
      match fs::copy(from, to) {
        Ok(_) => {},
        Err(error) => panic!("Error: failed copy of \"{}\" to \"{}\"\n\n{}", from, to, error)
      }
    }
  }

  pub fn empty_dir(dir_path: &str) {
    if super::file_system::exists(dir_path) {
      fs::remove_dir_all(dir_path).expect(&format!("Failed to delete dir \"{}\"", dir_path));
      fs::create_dir(dir_path).expect(&format!("Failed to create dir \"{}\"", dir_path));
    }
  }

  // https://doc.rust-lang.org/std/fs/fn.create_dir_all.html
  pub fn ensure_dir(path: &str) {
    if !super::file_system::exists(path) {
      match fs::create_dir_all(path) {
        Ok(_) => {},
        Err(error) => panic!("Error: failed to create directory \"{}\"\n\n{}", path, error)
      }
    }
  }

  // https://stackoverflow.com/a/32384768/10415695
  pub fn exists(path: &str) -> bool {
    Path::new(path).exists()
  }
}

pub mod string {
  use regex::Regex;

  pub fn to_slug(value: &String) -> String {
    let mut result: String = value.to_lowercase();
    let non_alpha_numeric: Regex = Regex::new(r"[^a-zA-Z0-9]+").unwrap();
    let slug_delimiter: Regex = Regex::new(r"[\s]+").unwrap();
    result = non_alpha_numeric.replace_all(&result, " ").trim().to_string();
    result = slug_delimiter.replace_all(&result, "-").to_string();
    result
  }

  pub fn escape_html_quotes(html: &String) -> String {
    html.replace("'", "&apos;").replace("\"", "&quot;")
  }
}

pub mod walk {
  use glob::glob;

  pub fn dir(dir_path: &str) -> Vec<String> {
    let error: &str = &format!("Error: failed to glob \"{}\"", dir_path);
    glob(dir_path)
      .expect(error)
      .map(| path |
        path.expect(error)
            .display()
            .to_string()
      )
      .collect::<Vec<String>>()
  }
}