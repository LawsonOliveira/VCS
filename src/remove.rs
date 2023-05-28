/// Provides functions for file removal operations.
use std::fs;
use std::io;

use crate::log;

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
fn verify_if_file_is_added(line_to_verify: &str, file_path: &str) -> Result<bool, std::io::Error> {
    let file_content = fs::read_to_string(file_path)?;

    let lines: Vec<&str> = file_content.lines().collect();

    Ok(lines.contains(&line_to_verify))
}

/// Removes a file from the file system.
///
/// # Arguments
///
/// * `file_path` - The path of the file to remove.
///
/// # Errors
///
/// Returns an error if there is an issue removing the file.
fn remove_file(file_path: &str) -> io::Result<()> {
    fs::remove_file(file_path)?;
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
pub fn remove(file_to_remove: &str, use_log: bool) -> Result<(), Box<dyn std::error::Error>> {
    let path = "my_vcs/";
    let line_to_remove = file_to_remove.to_string();
    let file_path2remove = format!("{}add_contents/{}.yml", path, file_to_remove);
    let staging_area_path = format!("{}staging_area.yml", path);

    // Create the staging area file if it doesn't exist
    if fs::metadata(&staging_area_path).is_err() {
        let _ = fs::File::create(&staging_area_path);
    }

    // Check if the file is already added
    match verify_if_file_is_added(&line_to_remove, &staging_area_path) {
        Ok(true) => {
            // Remove the line from the staging area file
            if let Err(error) = remove_line_from_file(&line_to_remove, &staging_area_path) {
                return Err(format!("Error removing line from file: {}", error).into());
            } else {
                if let Err(err) = remove_file(&file_path2remove) {
                    return Err(format!("Error removing file: {}", err).into());
                }
                if use_log {
                    log::start(format!("remove {}", file_to_remove));
                }
            }
        }
        Ok(false) => println!("File is not in the staging area.\n"),
        Err(error) => return Err(format!("Error verifying file existence: {}", error).into()),
    }

    Ok(())
}

