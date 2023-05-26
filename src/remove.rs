pub mod remove_fn {
    // PACKAGES
    use std::fs;
    use crate::log::logger;
    // Function to remove a line from a file
    fn remove_line_from_file(line_to_remove: &str, file_path: &str) -> Result<(), std::io::Error> {
        let file_content = fs::read_to_string(file_path)?;

        let lines: Vec<&str> = file_content.lines().filter(|&line| line != line_to_remove).collect();

        let modified_content = lines.join("\n");

        fs::write(file_path, modified_content)?;

        Ok(())
    }

    // Function to verify if a file is already added
    fn verify_if_file_is_added(line_to_verify: &str, file_path: &str) -> Result<bool, std::io::Error> {
        let file_content = fs::read_to_string(file_path)?;

        let lines: Vec<&str> = file_content.lines().collect();

        Ok(lines.contains(&line_to_verify))
    }

    // Function to remove a file from the staging area
    pub fn remove(file_to_remove: &str) {
        let path = "./my_vcs/saves/";
        let file_path2remove = format!("{}{}", path, file_to_remove);
        let staging_area_path = format!("{}staging_area.yml", path);

        // Create the staging area file if it doesn't exist
        if fs::metadata(&staging_area_path).is_err() {
            let _ = fs::File::create(&staging_area_path);
        }

        // Check if the file is already added
        match verify_if_file_is_added(&file_path2remove, &staging_area_path) {
            Ok(true) => {
                // Remove the line from the staging area file
                if let Err(error) = remove_line_from_file(&file_path2remove, &staging_area_path) {
                    eprintln!("Error removing line from file: {}", error);
                } else {
                    println!("File removed from the staging area.");
                    logger::start(format!("remove {}", file_to_remove).to_string());

                }
            }
            Ok(false) => println!("File is not in the staging area.\n"),
            Err(error) => eprintln!("Error verifying file existence: {}", error),
        }
    }
}
