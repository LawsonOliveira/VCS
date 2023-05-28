/// Provides functions for logging actions in the VCS.
use std::fs;
use std::io::Write;
use serde_yaml;

use crate::structs::structs_mod::Log;
use crate::structs::read_struct_from_file;
use crate::structs::update_struct_file;


use crate::time;

/// Starts logging the specified action.
///
/// # Arguments
///
/// * `action` - The action to log.
///
/// # Errors
///
/// Returns an error if there is an issue reading or updating the log file.
pub fn start(action: String) -> Result<(), Box<dyn std::error::Error>> {
    let time_date: [String; 2] = time::start();

    let action = action;
    let created_date = time_date[0].clone();
    let created_time = time_date[1].clone();

    let path = "my_vcs/";

    let mut log_struct: Log = read_struct_from_file(&format!("{}{}", path, "log.yml"))?;
    log_struct.action.push(action);
    log_struct.created_date.push(created_date);
    log_struct.created_time.push(created_time);

    // Update the struct file
    update_struct_file(&format!("{}{}", path, "log.yml"), &log_struct)?;

    Ok(())
}

