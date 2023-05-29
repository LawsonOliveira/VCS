/// Provides functions for file removal operations.
use std::fs;
use std::io;

use crate::log;
use crate::add::{verify_if_file_is_not_added, replace_line_in_file};

/// Removes a specific line from a file.
///
/// # Arguments
///
/// * `line_to_remove` - The line to be removed.
/// * `file_path` - The path of the file.
///
/// # Errors
///
/// Returns an error if there is an issue reading or writing the file.
fn remove_line_from_file(line_to_remove: &str, file_path: &str) -> Result<(), std::io::Error> {
    let file_content = fs::read_to_string(file_path)?;

    let lines: Vec<&str> = file_content.lines().filter(|&line| line != line_to_remove).collect();

    let modified_content = lines.join("\n");

    fs::write(file_path, modified_content)?;

    Ok(())
}

/// Verifies if a file is already added.
///
/// # Arguments
///
/// * `line_to_verify` - The line to verify.
/// * `file_path` - The path of the file.
///
/// # Errors
///
/// Returns an error if there is an issue reading the file.


/// Removes a file from the file system.
///
/// # Arguments
///
/// * `file_path` - The path of the file to remove.
///
/// # Errors
///
/// Returns an error if there is an issue removing the file.
pub fn remove_file(file_path: &str) -> io::Result<()> {
    fs::remove_file(file_path)?;
    println!("File removed successfully.");
    Ok(())
}

/// Removes a file from the staging area.
///
/// # Arguments
///
/// * `file_to_remove` - The file to remove.
///
/// # Errors
///
/// Returns an error if there is an issue removing the file or updating the log.
pub fn remove(filename: &str, use_log: bool) -> Result<(), Box<dyn std::error::Error>> {
    let path = "my_vcs/";
    let staging_area_path = format!("{}staging_area.yml", path);
    let file_path2remove = format!("{}add_contents/{}.yml", path, filename);


    // Check if the file is already added
    match verify_if_file_is_not_added(&filename, &staging_area_path) {
        Ok(true) => println!("File is not in the staging area.\n"),
        Ok(false) => {
            // Remove the line from the staging area file
            if let Err(error) = replace_line_in_file(&filename, "", &staging_area_path) {
                return Err(format!("Error removing line from file: {}", error).into());
            } else {
                if let Err(err) = remove_file(&file_path2remove) {
                    return Err(format!("Error removing file: {}", err).into());
                }
                println!("File removed from the staging area.");
                if use_log {
                    log::start(format!("remove {}", filename));
                }
            }
        },
        Err(error) => return Err(format!("Error verifying file existence: {}", error).into()),
    }

    Ok(())
}
