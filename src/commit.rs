use std::fs;
use std::path::Path;
use std::io::{BufRead, BufReader, Read};
use serde::{Serialize, Deserialize};
use sha2::{Digest, Sha256};
use std::collections::HashMap;

use diffy::{apply, Patch, PatchFormatter};

use serde_yaml;

use crate::log;
use crate::structs;
use crate::remove;
use crate::add::calculate_file_hash;
use crate::structs::structs_mod::{BranchChangesLog, CommitFiles, FileChangeLog};


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




fn create_file_patch_hashmap(vec_commits_files: &Vec<CommitFiles>) -> Result<HashMap<String, String>, Box<dyn std::error::Error>> {

    let mut file_patch_map: HashMap<String, String> = HashMap::new();

    for file_change_logs in vec_commits_files{
        for file_change_log in &file_change_logs.files_changelogs {
            let original_file = &file_change_log.original_file;
            if !file_patch_map.contains_key(original_file)  {
                let diff_path = format!("{}{}",file_change_log.hash_files_path,file_change_log.hash_changelog);

                let diff_content = fs::read_to_string(&diff_path).expect("could not read hash file");
                let patch: Patch<str> = Patch::from_str(&diff_content).unwrap();
                let new_value = apply("", &patch).unwrap();
                file_patch_map.insert(original_file.clone(), new_value);
            }

            else{
                let diff_path = format!("{}{}",file_change_log.hash_files_path,file_change_log.hash_changelog);
                let diff_content = fs::read_to_string(&diff_path).expect("could not read hash file");
                let patch: Patch<str> = Patch::from_str(&diff_content).unwrap();
                let previous_value: &mut String = file_patch_map.get_mut(original_file).unwrap();
                
                let new_value = apply(&previous_value.to_owned(), &patch).unwrap();
                file_patch_map.insert(original_file.clone(), new_value.clone());
            }
        }   
    }
    return Ok(file_patch_map);
}




fn push_tracked_file(line: &str, path: &str, branch_changes_log: &BranchChangesLog, new_commit_files: &mut CommitFiles) -> Result<(), Box<dyn std::error::Error>> {
    let binding = line.to_string();
    let splited_file_name_and_hash: Vec<&str>  = binding.split(":").collect();
    let filename_to_commit = splited_file_name_and_hash[0].trim().to_string();
    let filehash_to_commit = splited_file_name_and_hash[1].trim().to_string();
    let file_path = format!("{}add_contents/{}.yml", path, filename_to_commit);
    let file_last_patch_hashmap = create_file_patch_hashmap(&branch_changes_log.commits_files)?;
    if file_last_patch_hashmap.contains_key(&filename_to_commit) {
        let mut original_file: String = String::new();
        let mut last_file: String = String::new();
        let mut original_file_path: String = String::new();
        let mut hash_files_path: String = String::new();
        for commit_files in &branch_changes_log.commits_files {
            for file_change_log in &commit_files.files_changelogs {
                if filename_to_commit == file_change_log.original_file {
                    original_file = file_change_log.original_file.clone();
                    last_file = file_change_log.last_file.clone();
                    original_file_path = file_change_log.original_file_path.clone();
                    hash_files_path = file_change_log.hash_files_path.clone();
                }
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
        let previous_version: String = file_last_patch_hashmap.get(&filename_to_commit).ok_or_else(|| -> String{format!("Previous version not found for: {}", filename_to_commit).into()})?.to_string();

        // Gen dff
        let file = fs::File::create(filename_save_path)?;
        let patch = diffy::create_patch(&previous_version, &content_to_commit);
        PatchFormatter::new().write_patch_into(&patch, &file);

        new_commit_files.files_changelogs.push(new_file_change_log);
        // Remove the last version of the file
        let use_log = false;
        remove::remove(&filename_to_commit, use_log);
    }

    Ok(())
}



fn track_and_push_file(line: &str, path: &str, original_file_path: &str, new_commit_files: &mut CommitFiles) -> Result<(), Box<dyn std::error::Error>> {
    let binding = line.to_string();
    let splited_file_name_and_hash: Vec<&str>  = binding.split(":").collect();
    let filename_to_commit = splited_file_name_and_hash[0].trim().to_string();
    let filehash_to_commit = splited_file_name_and_hash[1].trim().to_string();
    let file_path = format!("{}add_contents/{}.yml", path, filename_to_commit);


    let new_file_change_log = structs::structs_mod::FileChangeLog {
        original_file: String::from(&filename_to_commit),
        last_file: String::from(&filename_to_commit),
        original_file_path: String::from(original_file_path),
        hash_files_path: String::from(format!("{}{}", path, "saves/")),
        hash_changelog: String::from(filehash_to_commit),
    };

    let filename_save_path = format!("{}{}", new_file_change_log.hash_files_path, new_file_change_log.hash_changelog);
    let content_to_commit = fs::read_to_string(&file_path)?;

    let file = fs::File::create(filename_save_path)?;
    let patch = diffy::create_patch("", &content_to_commit);
    PatchFormatter::new().write_patch_into(&patch, &file);

    // Remove the last version of the file
    let use_log = false;
    remove::remove(&filename_to_commit, use_log);
    new_commit_files.files_changelogs.push(new_file_change_log);

    Ok(())
}



pub fn commit(message: &str) -> Result<(), Box<dyn std::error::Error>> {
    let path = "my_vcs/";
    let staging_area_path = format!("{}staging_area.yml", path);
    let branch_name = "./main/";

    let commit_hash = calculate_file_hash(&staging_area_path)?;
    let mut new_commit_files = CommitFiles {
        files_changelogs: Vec::new(),
        commit_hash: String::new(),
    };

    let mut branch_changes_log: structs::structs_mod::BranchChangesLog =
    structs::StructWriter::read_struct_from_file(&format!("{}{}", path, "branch_changes_log.yml"))?;

    // Commit a tracked file
    let lines = read_file_lines(&staging_area_path)?;
    for line in lines { push_tracked_file(&line, &path, &branch_changes_log, &mut new_commit_files);}

    // Commit new files
    let lines = read_file_lines(&staging_area_path)?;
    for line in lines { track_and_push_file(&line,  &path, "./", &mut new_commit_files);}

    // Update branch_changes_log
    new_commit_files.commit_hash = commit_hash.clone();
    if branch_changes_log.branch_name == "Default initialization" {
        branch_changes_log.branch_name = branch_name.to_string();
        branch_changes_log.commits_files = vec![new_commit_files.clone()];
    }
    else{ branch_changes_log.commits_files.push(new_commit_files);}
    structs::StructWriter::update_struct_file(&format!("{}{}", path, "branch_changes_log.yml"), &branch_changes_log)?;
    log::start(format!("commit {}\nhash: {}", message, commit_hash));


    Ok(())
}