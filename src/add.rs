pub mod add_fn {
    // PACKAGES
    use std::fs;
    use std::io::Write;
    use std::io;
    use crate::log::logger;
    use std::collections::HashMap;
    use sha2::{Sha256, Digest};
    use std::io::Read;
    use hex;
    use std::path::Path;
    // Function to write a line to a file
    fn write_line(line_path: &str, file_path: &str) -> Result<(), std::io::Error> {
        // Open the file in append mode
        let mut file = fs::OpenOptions::new()
            .write(true)
            .append(true)
            .open(file_path)?;

        // Write the new line to the file
        write!(file, "{}", line_path)?;

        Ok(())
    }



    fn copy_file(source_path: &str, destination_path: &str) -> io::Result<()> {
        let source = Path::new(source_path);
        let destination = Path::new(destination_path);
    
        // Check if the source file exists
        if source.exists() && source.is_file() {
            // Create the destination directory if it doesn't exist
            if let Some(parent) = destination.parent() {
                fs::create_dir_all(parent)?;
            }
    
            // Copy the file
            fs::copy(source, destination)?;
            //println!("File copied successfully.");
        } else {
            return Err(io::Error::new(
                io::ErrorKind::NotFound,
                "Source file does not exist",
            ));
        }
    
        Ok(())
    }



    // Function to verify if a file exists
    fn verify_file_exists(file_path: &str) -> Result<(), std::io::Error> {
        if fs::metadata(file_path).is_ok() {
            Ok(())
        } else {
            Err(std::io::Error::new(
                std::io::ErrorKind::NotFound,
                "File does not exist",
            ))
        }
    }

    // Function to verify if a file is not already added
    fn verify_if_file_is_not_added(line_to_verify: &str, file_path: &str) -> Result<bool, std::io::Error> {
        let file_content = fs::read_to_string(file_path)?;

        let lines: Vec<&str> = file_content.lines().collect();

        Ok(!lines.contains(&line_to_verify))
    }

    // Function to calculate the hash value of a file
    fn calculate_file_hash(file_path: &str) -> Result<String, std::io::Error> {
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

    // Function to add a file to the staging area
    pub fn add(file_name: &str) {
        let path = "./my_vcs/";
        let file_path2add = format!("{}{}", "./", file_name);
//        let file_path2add = format!("{}{}", path, file_name);
        let staging_area_path = format!("{}staging_area.yml", path);

        // Create the staging area file if it doesn't exist
        if fs::metadata(&staging_area_path).is_err() {
            let _ = fs::File::create(&staging_area_path);
        }





        match verify_if_file_is_not_added(&file_path2add, &staging_area_path) {
            Ok(is_not_added) => {
                if is_not_added {
                    if let Err(error) = write_line(&file_path2add, &staging_area_path) {
                        eprintln!("Error adding file: {}", error);
                    } else {
                        copy_file(&file_path2add, &format!("{}/adds_contents/{}.yml", &path, &file_name));
                        logger::start(format!("add {}", file_name).to_string());
                        println!("File added successfully.");
                    }
                } else {
                    copy_file(&file_path2add, &format!("{}/adds_contents/{}.yml", &path, &file_name));
                    println!("File version updated in the stranging area");
                }
            }
            Err(error) => eprintln!("Error verifying file: {}", error),
        }








        /*match verify_if_file_is_not_added(&file_path2add, &staging_area_path) {
            Ok(is_not_added) => {
                if is_not_added {
                    // Calculate the hash value of the file
                    let file_hash = match calculate_file_hash(&file_path2add) {
                        Ok(hash) => hash,
                        Err(error) => {
                            eprintln!("Error calculating file hash: {}", error);
                            return;
                        }
                    };

                    // Add the file to the staging area with its hash value
                    let mut files_to_commit = HashMap::new();
                    files_to_commit.insert(file_path2add.clone(), file_hash.clone());

                    // Serialize the HashMap to YAML
                    let yaml_content = match serde_yaml::to_string(&files_to_commit) {
                        Ok(content) => content,
                        Err(error) => {
                            eprintln!("Error serializing to YAML: {}", error);
                            return;
                        }
                    };

                    // Write the YAML content to the staging area file
                    if let Err(error) = write_line(&yaml_content, &staging_area_path) {
                        eprintln!("Error adding file: {}", error);
                    } else {
                        copy_file(&file_path2add, &format!("{}/adds_contents/{}.yml", &path, &file_hash));
                        logger::start(format!("add {}", file_name).to_string());
                        println!("File added successfully.");

                    }
                } else {
                    println!("File is already added.");
                }

            }
            Err(error) => eprintln!("Error verifying file: {}", error),
        }*/
    }
}
