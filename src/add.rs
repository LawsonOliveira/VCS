use std::fs;
use std::io::{self, Read, Write};
use std::path::Path;
use sha2::{Digest, Sha256};
use hex;
use crate::log;

/// Write a line to a file.
///
/// This function opens the file in append mode and writes the new line to the file.
///
/// # Arguments
///
/// * `line` - The line to write to the file.
/// * `file_path` - The path of the file to write to.
///
/// # Errors
///
/// Returns an `Err` if there was an error writing to the file.
pub fn write_line(line: &str, file_path: &str) -> io::Result<()> {
    let mut file = fs::OpenOptions::new()
        .write(true)
        .append(true)
        .open(file_path)?;

    write!(file, "{}\n", line)?;

    Ok(())
}

/// Copy a file from source to destination.
///
/// This function copies the file from the source path to the destination path.
/// If the source file does not exist, an error is returned.
///
/// # Arguments
///
/// * `source_path` - The path of the source file.
/// * `destination_path` - The path of the destination file.
///
/// # Errors
///
/// Returns an `Err` if the source file does not exist or there was an error copying the file.
pub fn copy_file(source_path: &str, destination_path: &str) -> io::Result<()> {
    let source = Path::new(source_path);
    let destination = Path::new(destination_path);

    if source.exists() && source.is_file() {
        if let Some(parent) = destination.parent() {
            fs::create_dir_all(parent)?;
        }

        fs::copy(source, destination)?;
        Ok(())
    } else {
        Err(io::Error::new(
            io::ErrorKind::NotFound,
            "Source file does not exist",
        ))
    }
}

/// Verify if a file exists.
///
/// This function checks if the file exists at the given file path.
///
/// # Arguments
///
/// * `file_path` - The path of the file to verify.
///
/// # Errors
///
/// Returns an `Err` if the file does not exist.
pub fn verify_file_exists(file_path: &str) -> io::Result<()> {
    if fs::metadata(file_path).is_ok() {
        Ok(())
    } else {
        Err(io::Error::new(
            io::ErrorKind::NotFound,
            "File does not exist",
        ))
    }
}

/// Verify if a file is not already added.
///
/// This function checks if the given line is already present in the file.
/// It reads the file content, splits it into lines, and checks if the line exists.
///
/// # Arguments
///
/// * `line` - The line to verify.
/// * `file_path` - The path of the file to check.
///
/// # Errors
///
/// Returns an `Err` if there was an error reading the file.
pub fn verify_if_file_is_not_added(line: &str, file_path: &str) -> io::Result<bool> {
    let file_content = fs::read_to_string(file_path)?;

    let lines: Vec<&str> = file_content.lines().collect();

    Ok(!lines.contains(&line))
}

/// Calculate the hash value of a file.
///
/// This function calculates the SHA-256 hash value of the file at the given file path.
///
/// # Arguments
///
/// * `file_path` - The path of the file to calculate the hash for.
///
/// # Errors
///
/// Returns an `Err` if there was an error reading the file or calculating the hash.
pub fn calculate_file_hash(file_path: &str) -> io::Result<String> {
    let mut file = fs::File::open(file_path)?;
    let mut hasher = Sha256::new();
    let mut buffer = [0; 1024];

    loop {
        let bytes_read = file.read(&mut buffer)?;
        if bytes_read == 0 {
            break;
        }
        hasher.update(&buffer[..bytes_read]);
    }

    let result = hasher.finalize();
    let hash = hex::encode(result);

    Ok(hash)
}

/// Add a file to the version control system.
///
/// This function adds the specified file to the version control system.
/// It writes the file name to the staging area file, copies the file to the adds_contents directory,
/// and logs the add operation.
///
/// # Arguments
///
/// * `file_name` - The name of the file to add.
///
/// # Errors
///
/// Returns an `Err` if there was an error adding the file.
pub fn add_to_version_control(file_name: &str) -> io::Result<()> {
    let base_path = "my_vcs/";
    let file_path = file_name.to_owned();
    let staging_area_path = format!("{}staging_area.yml", base_path);
    let adds_contents_path = format!("{}/add_contents/{}.yml", base_path, file_name);

    // Create the staging area file if it doesn't exist
    if fs::metadata(&staging_area_path).is_err() {
        fs::File::create(&staging_area_path)?;
    }

    let is_not_added = verify_if_file_is_not_added(&file_path, &staging_area_path)?;

    if is_not_added {
        write_line(&file_path, &staging_area_path)?;
        copy_file(&file_path, &adds_contents_path)?;
        log::start(format!("add {}", file_name));
        println!("File added successfully.");
    } else {
        copy_file(&file_path, &adds_contents_path)?;
        println!("File version updated in the staging area");
    }

    Ok(())
}
