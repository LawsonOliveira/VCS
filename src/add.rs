pub mod add_fn {
    // PACKAGES
    use std::fs;
    use std::io::Write;

    // Function to write a line to a file
    fn write_line(line: &str, file_path: &str) -> Result<(), std::io::Error> {
        // Open the file in append mode
        let line_path = format!("./my_vcs/saves/{}", line);
        let mut file = fs::OpenOptions::new()
            .write(true)
            .append(true)
            .open(file_path)?;

        // Write the new line to the file
        writeln!(file, "{}", line_path)?;

        Ok(())
    }

    // Function to verify if a file is not already added
    fn verify_if_file_is_not_added(file_name: &str, file_path: &str) -> Result<bool, std::io::Error> {
        let line_to_verify = format!("./my_vcs/saves/{}", file_name);
        // Read the entire file into a string
        let file_content = fs::read_to_string(file_path)?;

        let lines: Vec<&str> = file_content.lines().collect();

        Ok(!lines.contains(&line_to_verify.as_str()))
    }

    // Function to add a file to the staging area
    pub fn add(file_name: &str) {
        let staging_area_path = "./my_vcs/saves/staging_area.yaml";

        // Create the staging area file if it doesn't exist
        if fs::metadata(staging_area_path).is_err() {
            let _ = fs::File::create(staging_area_path);
        }

        // Check if the file is not already added
        match verify_if_file_is_not_added(file_name, staging_area_path) {
            Ok(true) => {
                // Write the file name to the staging area
                if let Err(error) = write_line(file_name, staging_area_path) {
                    eprintln!("Error writing to file: {}", error);
                }
            }
            Ok(false) => println!("File is already in the staging area.\n"),
            Err(error) => eprintln!("Error verifying file existence: {}", error),
        }
    }
}
