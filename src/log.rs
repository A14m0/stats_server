// defines all logging related functions
use colored::Colorize;
use chrono::prelude::*;

/// defines the log type enumerator
pub enum LTYPE {
    Info,
    Warn,
    Error
}

/// defines the log function
pub fn log(lt: LTYPE, dat: String) {
    // determine prologue text and color
    let prolog = match lt {
        LTYPE::Info => "[INFO]".green(),
        LTYPE::Warn => "[WARN]".yellow(),
        LTYPE::Error => "[FATAL]".red().bold()
    };

    // get the date
    let dt = Local::now();
    let fmt = dt.format("%m-%d %H:%M:%S");//.to_string();

    // print it
    println!("{} [{}] {}", prolog, fmt, dat);
}