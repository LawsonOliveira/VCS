pub mod add_fn {
    // PACKAGES
    use std::fs;
    use std::io::Write;

    // Function to write a line to a file
    fn write_line(line_path: &str, file_path: &str) -> Result<(), std::io::Error> {
        // Open the file in append mode
        let mut file = fs::OpenOptions::new()
            .write(true)
            .append(true)
            .open(file_path)?;

        // Write the new line to the file
        writeln!(file, "{}", line_path)?;

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

    // Function to add a file to the staging area
    pub fn add(file_name: &str) {
        let path = "./my_vcs/saves/";
        let file_path2add = format!("{}{}", path, file_name);
        let staging_area_path = format!("{}staging_area.yaml", path);

        // Create the staging area file if it doesn't exist
        if fs::metadata(&staging_area_path).is_err() {
            let _ = fs::File::create(&staging_area_path);
        }

        let result = verify_file_exists(&file_path2add).and_then(|_| {
            verify_if_file_is_not_added(&file_path2add, &staging_area_path)
        });

        match result {
            Ok(is_not_added) => {
                if is_not_added {
                    if let Err(error) = write_line(&file_path2add, &staging_area_path) {
                        eprintln!("Error adding file: {}", error);
                    } else {
                        println!("File added successfully.");
                    }
                } else {
                    println!("File is already added.");
                }
            }
            Err(error) => eprintln!("Error verifying file: {}", error),
        }
    }
}
