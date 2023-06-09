use std::fs;
use std::io::{self, Read, Write};
use std::path::Path;
use sha2::{Digest, Sha256};
use hex;

use crate::log;


/// Write a line to a file.
/// Returns `Ok(())` if the line was written successfully, otherwise returns an `Err` with the corresponding error.
pub fn write_line(line: &str, file_path: &str) -> Result<(), std::io::Error> {
    let mut file = fs::OpenOptions::new()
        .write(true)
        .append(true)
        .open(file_path)?;

    write!(file, "{}\n", line)?;

    Ok(())
}

/// Copy a file from source to destination.
/// Returns `Ok(())` if the file was copied successfully, otherwise returns an `Err` with the corresponding error.
pub fn copy_file(source_path: &str, destination_path: &str) -> Result<(), std::io::Error>{
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
/// Returns `Ok(true)` if the file exists, otherwise returns an `Err` with the corresponding error.
pub fn verify_file_exists(file_path: &str) -> Result<bool, std::io::Error> {
    if fs::metadata(file_path).is_ok() {
        Ok(true)
    } else {
        Err(io::Error::new(
            io::ErrorKind::NotFound,
            "File does not exist",
        ))
    }
}

/// Verify if a file is not already added.
/// Returns `Ok(true)` if the file is not already added, otherwise returns an `Err` with the corresponding error.
pub fn verify_if_file_is_not_added(file_name: &str, file_path: &str) -> Result<bool, std::io::Error> {
    let file_content = fs::read_to_string(file_path)?;
    let pattern = format!(r"\b{}\b", regex::escape(file_name));
    let regex = regex::Regex::new(&pattern).unwrap();

    Ok(!regex.is_match(&file_content))
}


/// Calculates the hash value of a file.
/// Returns `Ok(hash)` if the hash was calculated successfully, otherwise returns an `Err` with the corresponding error.
pub fn calculate_file_hash(file_path: &str) -> Result<String, std::io::Error> {
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



/// Replace the respective line in the file.
/// Returns `Ok(())` if the line was replaced successfully, otherwise returns an `Err` with the corresponding error.
pub fn replace_line_in_file(filename: &str, replacement: &str, file_path: &str) -> Result<(), std::io::Error> {
    let search_pattern = format!("{}: {}", filename, "[a-f0-9]+\n");
    let content = fs::read_to_string(file_path)?;
    let regex = regex::Regex::new(&search_pattern).unwrap();
    let replaced_content = regex.replace_all(&content, replacement);

    fs::write(file_path, replaced_content.into_owned())?;

    Ok(())
}




/// Add a file to the version control system.
/// Returns `Ok(())` if the line was added to staging area, otherwise returns an `Err` with the corresponding error.
pub fn add_to_version_control(file_name: &str) -> Result<(), std::io::Error> {
    let file_exists = std::path::Path::new(&file_name).exists();
    if !file_exists {
        return Err(*Box::new(io::Error::new(io::ErrorKind::NotFound, "File does not exists")));
    }

    let base_path = "my_vcs/";
    let file_path = format!("{}{}", base_path, file_name);
    let staging_area_path = format!("{}staging_area.yml", base_path);
    let adds_contents_path = format!("{}/add_contents/{}.yml", base_path, file_name);
    let file_hash = calculate_file_hash(&file_name)?;
    let file_name_and_hash = format!("{}: {}", file_name, file_hash);


    // Create the staging area file if it doesn't exist
    if fs::metadata(&staging_area_path).is_err() {
        fs::File::create(&staging_area_path)?;
    }

    let is_not_added = verify_if_file_is_not_added(&file_name, &staging_area_path)?;

    if is_not_added {
        write_line(&file_name_and_hash, &staging_area_path);
        copy_file(&file_name, &adds_contents_path);
        log::start(format!("add {}", &file_name));
        println!("File added successfully.");
    } else {
        // update hash in the line
        replace_line_in_file(&file_name, &format!("{}\n", file_name_and_hash), &staging_area_path);
        copy_file(&file_name, &adds_contents_path);
        println!("File version updated in the staging area");
    }

    Ok(())
}
