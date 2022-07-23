#![allow(non_snake_case)]
pub mod FileTime {
  use chrono::{DateTime, format::{DelayedFormat, StrftimeItems}, NaiveDateTime, Utc};
  use std::{fs::{Metadata, metadata}, time::SystemTime};

  #[allow(dead_code)]
  fn tv_secs(system_time: SystemTime) -> i64 {
    // SystemTime { tv_sec: 1657846097, tv_nsec: 129747070 }
    let as_string: String = format!("{:?}", system_time);
    // "1657846097,129747070"
    let values: String = as_string.chars().filter(| c: &char | c.is_digit(10) || *c == ',').collect();
    // ["1657846097", "129747070"]
    let split_array: Vec<&str> = values.split(",").collect();
    // 1657846097
    split_array[0].parse().unwrap()
  }

  #[allow(dead_code)]
  fn format(system_time: SystemTime) -> String {
    // Get the "tv_secs" from the given system time
    let secs: i64 = self::tv_secs(system_time);
    // Create a NaiveDateTime from the timestamp
    let naive: NaiveDateTime = NaiveDateTime::from_timestamp(secs, 0);
    // Create a normal DateTime from the NaiveDateTime
    let datetime: DateTime<Utc> = DateTime::from_utc(naive, Utc);
    // Format the datetime how you want
    let newdate: DelayedFormat<StrftimeItems> = datetime.format("%d/%m/%Y at %H:%M:%S");
    // Return the string
    newdate.to_string()
  }

  #[allow(dead_code)]
  pub fn modified(file_path: &str) -> String {
    let metadata: Metadata = metadata(file_path).unwrap();
    self::format(metadata.modified().unwrap())
  }

  #[allow(dead_code)]
  pub fn created(file_path: &str) -> String {
    let metadata: Metadata = metadata(file_path).unwrap();
    self::format(metadata.created().unwrap())
  }
}

// Check for support by trying to execute and not logging if fails
// If a log file doesn't exist a new log file is created
// This log file contains the last modification times for each defined template
// The template will only render if:
//   the last modified time is not equal to the current modified time || Config::clean is set to true