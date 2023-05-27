pub mod commit_fn
{


    use std::path::Path;
    use crate::log::logger;
    use std::path::PathBuf;
    use std::io::BufRead;
	// PACKAGES
	use std::fs;
    use diffy;
    use std::io;
    use crate::structs::structs_mod;
    use serde_yaml;
    use sha2::{Digest, Sha256};
    use crate::io::BufReader;
    use std::collections::HashMap;

    use serde::{Serialize, Deserialize};
    use std::fs::{File, OpenOptions};
    use std::io::{Read, Write};


    fn read_structs_from_yaml<T: for<'de> Deserialize<'de>>(file_path: &str) -> Result<T, Box<dyn std::error::Error>> {
        let mut file = File::open(file_path)?;
        let mut contents = String::new();
        file.read_to_string(&mut contents)?;
    
        let result: T = serde_yaml::from_str(&contents)?;
    
        Ok(result)
    }

    fn write_structs_to_yaml<T: Serialize>(structs: &T, file_path: &str) -> Result<(), Box<dyn std::error::Error>> {
        let yaml_string = serde_yaml::to_string(&structs)?;
    
        let mut file = OpenOptions::new()
            .write(true)
            .create(true)
            .truncate(true)
            .open(file_path)?;
    
        file.write_all(yaml_string.as_bytes())?;
    
        Ok(())
    }



    fn gen_diff(original_text_path: &str, modified_text_path: &str, save_path: &str) {
        // Read the contents of the original text file
        let original_text = match fs::read_to_string(original_text_path) {
            Ok(text) => text,
            Err(error) => {
                eprintln!("Error reading original text file: {}", error);
                return;
            }
        };
    
        // Read the contents of the modified text file
        let modified_text = match fs::read_to_string(modified_text_path) {
            Ok(text) => text,
            Err(error) => {
                eprintln!("Error reading modified text file: {}", error);
                return;
            }
        };
    
        // Create a patch based on the differences between the original and modified texts
        let patch = diffy::create_patch(&original_text, &modified_text);
    
        // Create a new file for writing the patch
        let file = match fs::File::create(save_path) {
            Ok(file) => file,
            Err(error) => {
                eprintln!("Error creating file: {}", error);
                return;
            }
        };
    
        // Write the patch into the file
        if let Err(error) = diffy::PatchFormatter::new().write_patch_into(&patch, &file) {
            eprintln!("Error writing diff to file: {}", error);
            return;
        }
    }


    fn apply_patch(original_text_path: &str, diff_patch_path: &str, save_path: &str) {
        // Read the contents of the original text file
        let original_text = match fs::read_to_string(original_text_path) {
            Ok(text) => text,
            Err(error) => {
                eprintln!("Error reading original file: {}", error);
                return;
            }
        };
    
        // Read the contents of the patch file
        let patch_text = match fs::read_to_string(diff_patch_path) {
            Ok(text) => text,
            Err(error) => {
                eprintln!("Error reading patch file: {}", error);
                return;
            }
        };
    
        // Parse the patch text into a Patch object
        let patch = match diffy::Patch::from_str(&patch_text) {
            Ok(patch) => patch,
            Err(error) => {
                eprintln!("Error parsing patch: {}", error);
                return;
            }
        };
    
        // Apply the patch to the original text
        let new_text_version = match diffy::apply(&original_text, &patch) {
            Ok(new_text) => new_text,
            Err(error) => {
                eprintln!("Error applying patch: {}", error);
                return;
            }
        };
    
        // Write the new version of the text to a file
        if let Err(error) = fs::write(save_path, new_text_version) {
            eprintln!("Error writing new version: {}", error);
            return;
        }
    }


    fn read_file_lines(file_path: &str) -> Result<Vec<String>, std::io::Error> {
        let file = File::open(file_path)?;
        let reader = BufReader::new(file);
        let lines: Result<Vec<String>, _> = reader.lines().collect();
        Ok(lines?)
    }
        



// Function to commit changes
pub fn commit(message: &str) {
    let path = "./my_vcs/";
    let staging_area_path = format!("{}staging_area.yml", path);
    let commit_path = format!("{}commits/", path);



    match read_file_lines(&staging_area_path) {
        Ok(lines) => {
            for line in lines {
            let file_path = format!("{}adds_contents/{}.yml", path, line);

                println!("{}", line);







            }
        }
        Err(err) => eprintln!("Error reading file: {}", err),
    }



}
}


