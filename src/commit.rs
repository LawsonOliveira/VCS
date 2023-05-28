use std::fs;
use std::path::Path;
use std::io::{BufRead, BufReader, Read};
use serde::{Serialize, Deserialize};
use sha2::{Digest, Sha256};

use diffy;
use serde_yaml;

use crate::log;
use crate::structs;
use crate::remove;
use crate::add::calculate_file_hash;

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
pub fn commit(message: &str) -> Result<(), Box<dyn std::error::Error>> {
    let path = "my_vcs/";
    let staging_area_path = format!("{}staging_area.yml", path);
    // Update the branch_changes_log in the file or source
    let commit_hash = calculate_file_hash(&staging_area_path)?;

    let mut branch_changes_log: structs::structs_mod::BranchChangesLog =
    structs::StructWriter::read_struct_from_file(&format!("{}{}", path, "branch_changes_log.yml"))?;
    let mut committed_files: Vec<String> = Vec::new();

    
    match read_file_lines(&staging_area_path) {
        Ok(lines) => {
            for line in lines {  

                let splited_file_name_and_hash: Vec<&str>  = line.split(":").collect();
                let filename_to_commit = splited_file_name_and_hash[0].trim().to_string();
                let filehash_to_commit = splited_file_name_and_hash[1].trim().to_string();
                let file_path = format!("{}add_contents/{}.yml", path, filename_to_commit);
                committed_files.push(filename_to_commit.clone());


                // Find the file changelog in the branch_changes_log
                for file_changelog in &mut branch_changes_log.files_changelogs {
                    if file_changelog.original_file == filename_to_commit {
                        let mut mutable_file_changelog = file_changelog.clone();


                        // Apply the hash changelogs to the previous version
                        let mut previous_version = file_changelog.original_file.clone();
                        for hash_diff in &mutable_file_changelog.hash_changelog {
                            previous_version = apply_patch(
                                &previous_version,
                                &format!("{}{}.yml", mutable_file_changelog.hash_files_path, hash_diff),
                            )?;
                        }

                        // Generate the diff between the previous version and the content to commit
                        let filename_save_path = format!("{}{}", mutable_file_changelog.hash_files_path, filehash_to_commit);
                        let content_to_commit = fs::read_to_string(&file_path)?;
                        gen_diff(&previous_version, &content_to_commit, &filename_save_path);

                        mutable_file_changelog.last_file = filename_to_commit.clone();
                        mutable_file_changelog.hash_changelog.push(filehash_to_commit.clone());

                        // Remove the last version of the file
                        let use_log = false;
                        remove::remove(&filename_to_commit, use_log);

                        *file_changelog = mutable_file_changelog;

                        
                    }
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
                    hash_changelog: vec![filehash_to_commit.clone()],
                };

                let filename_save_path = format!("{}{}", new_file_change_log.hash_files_path, filehash_to_commit);
                let content_to_commit = fs::read_to_string(&file_path)?;
                gen_diff("", &content_to_commit, &filename_save_path);

                // Remove the last version of the file
                let use_log = false;
                remove::remove(&filename_to_commit, use_log);

                branch_changes_log.files_changelogs.push(new_file_change_log);
            }


        }
        Err(err) => return Err(format!("Error creating commit: {}", err).into()),
    }


    let new_commit_files = structs::structs_mod::CommitFiles {
        commit_files: committed_files,
        commit_hash: commit_hash.clone(),
    };
    branch_changes_log.commits_hash.push(new_commit_files);
    structs::StructWriter::update_struct_file(&format!("{}{}", path, "branch_changes_log.yml"), &branch_changes_log)?;
    log::start(format!("commit {}\nhash: {}", message, commit_hash));




    Ok(())
}
