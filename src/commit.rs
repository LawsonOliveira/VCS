use std::fs;
use std::path::Path;
use std::io::{BufRead, BufReader, Read};
use serde::{Serialize, Deserialize};
use sha2::{Digest, Sha256};
use std::collections::HashMap;

use diffy;
use serde_yaml;

use crate::log;
use crate::structs;
use crate::remove;
use crate::add::calculate_file_hash;
use crate::structs::structs_mod::{BranchChangesLog, CommitFiles, FileChangeLog};
/// Generates a diff patch between the original text and modified text and saves it to a file.
///
/// # Arguments
///
/// * `original_text` - The original text.
/// * `modified_text` - The modified text.
/// * `save_path` - The path to save the diff patch file.
///
fn gen_diff(original_text: &str, modified_text: &str, save_path: &str) {
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

/// Applies a patch to the original text and returns the modified text.
///
/// # Arguments
///
/// * `original_text` - The original text.
/// * `patch_text` - The patch text to apply.
///
/// # Returns
///
/// * The modified text after applying the patch.
/// * An error if parsing the patch or applying it fails.
///
fn apply_patch(original_text: &str, patch_text: &str) -> Result<String, Box<dyn std::error::Error>> {
    // Parse the patch text into a Patch object
    let patch = match diffy::Patch::from_str(&patch_text) {
        Ok(patch) => patch,
        Err(error) => {
            return Err(format!("Error parsing patch: {}", error).into());
        }
    };

    // Apply the patch to the original text
    let new_text_version = match diffy::apply(&original_text, &patch) {
        Ok(new_text) => new_text,
        Err(error) => {
            return Err(format!("Error applying patch: {}", error).into());
        }
    };

    Ok(new_text_version)
}

/// Reads the lines from a file and returns them as a vector of strings.
///
/// # Arguments
///
/// * `file_path` - The path to the file to read.
///
/// # Returns
///
/// * A vector of strings representing the lines read from the file.
/// * An error if reading the file fails.
///
fn read_file_lines(file_path: &str) -> Result<Vec<String>, std::io::Error> {
    let file = fs::File::open(file_path)?;
    let reader = BufReader::new(file);
    let lines: Result<Vec<String>, _> = reader.lines().collect();
    Ok(lines?)
}


/// Commits the changes.
///
/// # Arguments
///
/// * `message` - A string containing the commit message.
///
/// # Examples
///
/// ```
/// let message = "Initial commit";
/// match commit(message) {
///     Ok(()) => println!("Changes committed successfully."),
///     Err(err) => eprintln!("Error committing changes: {}", err),
/// }
/// ```



fn create_file_patch_hashmap(vec_commits_files: &Vec<CommitFiles>) -> Result<HashMap<String, String>, Box<dyn std::error::Error>> {

    let mut file_patch_map: HashMap<String, String> = HashMap::new();

    for file_change_logs in vec_commits_files{
        for file_change_log in &file_change_logs.files_changelogs {
            let original_file = &file_change_log.original_file;

            if !file_patch_map.contains_key(original_file) {
                file_patch_map.insert(original_file.clone(), String::new());
            }

            else{
                let value = file_patch_map.get_mut(original_file).unwrap();
                let new_value = apply_patch(value, &format!("{}{}.yml", file_change_log.hash_files_path, file_change_log.hash_changelog))?;
                value.push_str(&new_value);
                }

        }
    }
    return Ok(file_patch_map);
}




pub fn commit(message: &str) -> Result<(), Box<dyn std::error::Error>> {
    let path = "my_vcs/";
    let staging_area_path = format!("{}staging_area.yml", path);

    // Update the branch_changes_log in the file or source
    let mut branch_changes_log: structs::structs_mod::BranchChangesLog =
    structs::StructWriter::read_struct_from_file(&format!("{}{}", path, "branch_changes_log.yml"))?;

    let commit_hash = calculate_file_hash(&staging_area_path)?;

    let mut branch_changes_log: structs::structs_mod::BranchChangesLog =
    structs::StructWriter::read_struct_from_file(&format!("{}{}", path, "branch_changes_log.yml"))?;
    let file_last_patch_hashmap = create_file_patch_hashmap(&branch_changes_log.commits_files)?;

    let mut new_commit_files = CommitFiles {
        files_changelogs: Vec::new(),
        commit_hash: String::new(),
    };
    match read_file_lines(&staging_area_path) {
        Ok(lines) => {
            for line in lines {
                let splited_file_name_and_hash: Vec<&str> = line.split(":").collect();
                let filename_to_commit = splited_file_name_and_hash[0].trim().to_string();
                let filehash_to_commit = splited_file_name_and_hash[1].trim().to_string();
                let file_path = format!("{}add_contents/{}.yml", path, filename_to_commit);

                if file_last_patch_hashmap.contains_key(&filename_to_commit) {   
                    let mut original_file: String = String::new();
                    let mut last_file: String = String::new();
                    let mut original_file_path: String = String::new();
                    let mut hash_files_path: String = String::new();
                    for commit_files in &branch_changes_log.commits_files {
                        for file_change_log in &commit_files.files_changelogs{
                            original_file = file_change_log.original_file.clone();
                            last_file = file_change_log.original_file.clone();
                            original_file_path = file_change_log.original_file.clone();
                            hash_files_path = file_change_log.original_file.clone();
                        }    
                    }

                    let new_file_change_log = FileChangeLog {
                        original_file_path: original_file_path,
                        original_file: original_file,
                        hash_changelog: filehash_to_commit,
                        last_file: last_file,
                        hash_files_path: hash_files_path,
                    };


                    // Generate the diff between the previous version and the content to commit
                    let filename_save_path = format!("{}{}", new_file_change_log.hash_files_path, new_file_change_log.hash_changelog);
                    let content_to_commit = fs::read_to_string(&file_path)?;
                    let previous_version = file_last_patch_hashmap.get(&filename_to_commit).unwrap_or_else(|| {
                        // Handle the case where the key is not found
                        // You can return an error or handle it in a different way
                        panic!("Previous version not found for: {}", filename_to_commit);
                    });
                    gen_diff(&previous_version, &content_to_commit, &filename_save_path);


                    new_commit_files.files_changelogs.push(new_file_change_log);
                    // Remove the last version of the file
                    let use_log = false;
                    remove::remove(&filename_to_commit, use_log);


                }
            }
        }
        Err(err) => return Err(format!("Error reading file: {}", err).into()),
    }


    // Commit new files
    match read_file_lines(&staging_area_path) {
        Ok(lines) => {

            for line in lines {
                let splited_file_name_and_hash: Vec<&str>  = line.split(":").collect();
                let filename_to_commit = splited_file_name_and_hash[0].trim().to_string();
                let filehash_to_commit = splited_file_name_and_hash[1].trim().to_string();
                let file_path = format!("{}add_contents/{}.yml", path, filename_to_commit);


                let new_file_change_log = structs::structs_mod::FileChangeLog {
                    original_file: String::from(&filename_to_commit),
                    last_file: String::from(&filename_to_commit),
                    original_file_path: String::from("./"),
                    hash_files_path: String::from(format!("{}{}", path, "saves/")),
                    hash_changelog: String::from(filehash_to_commit),
                };

                let filename_save_path = format!("{}{}", new_file_change_log.hash_files_path, new_file_change_log.hash_changelog);
                let content_to_commit = fs::read_to_string(&file_path)?;
                gen_diff("", &content_to_commit, &filename_save_path);

                // Remove the last version of the file
                let use_log = false;
                remove::remove(&filename_to_commit, use_log);
                new_commit_files.files_changelogs.push(new_file_change_log);
            }


        }
        Err(err) => return Err(format!("Error creating commit: {}", err).into()),
    }


    new_commit_files.commit_hash = commit_hash.clone();
    branch_changes_log.commits_files.push(new_commit_files);
    structs::StructWriter::update_struct_file(&format!("{}{}", path, "branch_changes_log.yml"), &branch_changes_log)?;
    log::start(format!("commit {}\nhash: {}", message, commit_hash));




    Ok(())
}

