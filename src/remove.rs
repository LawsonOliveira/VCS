pub mod remove_fn {
    // PACKAGES
    use std::fs;

    // Function to remove a line from a file
    fn remove_line_from_file(file_to_remove: &str, file_path: &str) -> Result<(), std::io::Error> {
        let line_to_remove = format!("./my_vcs/saves/{}", file_to_remove);
        // Read the entire file into a string
        let file_content = fs::read_to_string(file_path)?;

        // Split the file content into lines
        let mut lines: Vec<&str> = file_content.lines().collect();

        // Find and remove the line matching the given text
        lines.retain(|&line| line != line_to_remove);

        // Join the remaining lines back into a single string
        let modified_content = lines.join("\n");

        // Write the modified content back to the file
        fs::write(file_path, modified_content)?;

        Ok(())
    }

    // Function to verify if a file is already added
    fn verify_if_file_is_added(file_name: &str, file_path: &str) -> Result<bool, std::io::Error> {
        let line_to_verify = format!("./my_vcs/saves/{}", file_name);
        // Read the entire file into a string
        let file_content = fs::read_to_string(file_path)?;

        let lines: Vec<&str> = file_content.lines().collect();

        Ok(lines.contains(&line_to_verify.as_str()))
    }

    // Function to remove a file from the staging area
    pub fn remove(file_to_remove: &str) {
        let staging_area_path = "./my_vcs/saves/staging_area.yaml";

        // Create the staging area file if it doesn't exist
        if fs::metadata(staging_area_path).is_err() {
            let _ = fs::File::create(staging_area_path);
        }

        // Check if the file is already added
        match verify_if_file_is_added(file_to_remove, staging_area_path) {
            Ok(true) => {
                // Remove the line from the staging area file
                if let Err(error) = remove_line_from_file(file_to_remove, staging_area_path) {
                    eprintln!("Error removing line from file: {}", error);
                }
            }
            Ok(false) => println!("File is not in the staging area.\n"),
            Err(error) => eprintln!("Error verifying file existence: {}", error),
        }
    }
}
