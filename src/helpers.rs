use chrono::prelude::*;

/// returns the current time as a string
pub fn gettime() -> String {
    // get the date
    let dt = Local::now();
    dt.format("%m-%d %H:%M:%S").to_string()
}