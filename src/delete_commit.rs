use std::fs;
use serde::{Deserialize, Serialize};
use serde_yaml::{self};
use crate::structs::structs_mod::{FileChangeLog, CommitFiles, BranchChangesLog};
use crate::structs;
use crate::log;

fn verify_if_commit_exist(commit_to_verify: &str, branch: &BranchChangesLog) -> Result<usize, std::io::Error> {
    let mut index: i32=-1;
    for (i, commit_file) in branch.commits_files.iter().enumerate() {
        if commit_file.commit_hash == commit_to_verify {
            index = i as i32;
        }
    }
    if index>=0{Ok(index as usize)}
    else {Err(std::io::Error::new(std::io::ErrorKind::NotFound, "Commit doesn't exist"))}
}

fn verify_if_left_commits_use_the_diff(commit_hash_to_verify: &str, left: &[CommitFiles]) -> bool {
    for commit_file in left {
        for file_change_log in &commit_file.files_changelogs {
            if file_change_log.hash_changelog == commit_hash_to_verify {
                return true;
            }
        }
    }
    false
}

// Function to remove a file from the staging area
pub fn delete(commit_to_remove: &str) -> Result<String, Box<dyn std::error::Error>> {
    let path = "my_vcs/";
    let mut branch: structs::structs_mod::BranchChangesLog =
        structs::StructWriter::read_struct_from_file(&format!("{}{}", path, "branch_changes_log.yml"))?;

    let index = verify_if_commit_exist(commit_to_remove, &branch);
    match verify_if_commit_exist(commit_to_remove, &branch) {
        Ok(index) => {
            let commits_files = &branch.commits_files;

            // Split the vector [hash_to_delete, last_hash] rest
            let (left, right) = commits_files.split_at(index);
        
            // Delete diff files
            for commit_file in right {
                for file_change_log in &commit_file.files_changelogs {
                    if !verify_if_left_commits_use_the_diff(&file_change_log.hash_changelog, &left) {
                        let file_to_delete = format!("{}{}", file_change_log.hash_files_path, file_change_log.hash_changelog);
                        if let Err(err) = fs::remove_file(file_to_delete) {
                            return Err(Box::new(err));
                        }
                    }
                }
            }
        
            branch.commits_files = left.to_vec();
            structs::StructWriter::update_struct_file(&format!("{}{}", path, "branch_changes_log.yml"), &branch)?;
            log::start(format!("delete {}", &commit_to_remove));
            println!("Commit with hash {} deleted", &commit_to_remove);
            Ok(format!("Commit with hash {} deleted ", commit_to_remove))

            
        }
        Err(err) => {
            // Handle the error case where the commit doesn't exist
            println!("Error: {}", err);
            Err(Box::new(err))        }
    }

}
