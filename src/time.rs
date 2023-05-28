use chrono::{Local, DateTime};

/// Get the current date in the format "%d/%m/%y".
fn get_date(local: DateTime<Local>) -> String {
    local.format("%d/%m/%y").to_string()
}

/// Get the current time in the format "%H:%M:%S".
fn get_time(local: DateTime<Local>) -> String {
    local.format("%H:%M:%S").to_string()
}

/// Get the current date and time as an array of strings.
pub fn start() -> [String; 2] {
    let local: DateTime<Local> = Local::now();
    [get_date(local), get_time(local)]
}

