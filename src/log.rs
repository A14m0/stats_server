// defines all logging related functions
use colored::Colorize;
use crate::helpers::gettime;

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

    
    // print it
    println!("{} [{}] {}", prolog, gettime(), dat);
}