use std::fs;
use serde::{Deserialize, Serialize};
use serde_yaml::{self};
use std::io;
use crate::structs::structs_mod::{FileChangeLog, Branch};
use crate::structs;
use crate::log;
use crate::commit::{build_commit_tree};


fn verify_if_commit_exist(commit_to_verify: &str, branch: &Branch) -> Result<bool, std::io::Error> {
    for commit in branch.commits.iter() {
        if commit.commit_hash == commit_to_verify {
            return Ok(true);
        }
    }
    Err(std::io::Error::new(std::io::ErrorKind::NotFound, "Commit doesn't exist"))
}


fn delete_files_not_in_vector(folder_path: &str, files_to_retain: &[String]) -> Result<(), io::Error> {
    let folder_entries = fs::read_dir(folder_path)?;

    for entry in folder_entries {
        let entry = entry?;
        let file_name = entry.file_name();
        let file_name_str = file_name.to_string_lossy().into_owned();

        if !files_to_retain.contains(&file_name_str) {
            let file_path = entry.path();
            if file_path.is_file() {
                fs::remove_file(file_path)?;
                println!("Deleted file: {}", file_name_str);
            }
        }
    }

    Ok(())
}


// Function to remove a file from the staging area
pub fn delete(commit_to_remove: &str) -> Result<String, Box<dyn std::error::Error>> {
    let path = "my_vcs/";
    let mut repository: structs::structs_mod::Repository =
        structs::StructWriter::read_struct_from_file(&format!("{}{}", &path, "my_repo.yml"))?;

    // 1. Find the branch in the repository based on the given branch name

    let branch = match repository.branches.iter_mut().find(|b| b.branch_name == repository.current_branch) {
        Some(branch) => branch,
        None => {
            println!("Branch not found");
            return Err(Box::new(io::Error::new(io::ErrorKind::NotFound, "Branch not found")));
        }
    };

    match verify_if_commit_exist(commit_to_remove, &branch) {
        Ok(index) => {
            let mut commit_tree = build_commit_tree(branch, commit_to_remove)?;
            if let Some((last, elements)) = commit_tree.split_last() {
                commit_tree = elements.to_vec();
            }
            let mut saves_to_retain: Vec<String> = Vec::new();
            
            // Collect saves to retain
            for commit in &commit_tree {
                for file_change_log in &commit.files_changelogs {
                    saves_to_retain.push(file_change_log.hash_changelog.clone());
                }
            }

            // Delete diff files not in saves_to_retain
            delete_files_not_in_vector(&format!("{}saves/", path), &saves_to_retain)?;

            branch.commits = commit_tree.to_vec();
            branch.head_commit_hash = commit_to_remove.to_string();
            structs::StructWriter::update_struct_file(&format!("{}{}", path, "my_repo.yml"), &repository)?;
            log::start(format!("delete {}", &commit_to_remove));
            println!("Commit with hash {} deleted", &commit_to_remove);
            Ok(format!("Commit with hash {} deleted ", commit_to_remove))
        }
        Err(err) => {
            // Handle the error case where the commit doesn't exist
            println!("Error: {}", err);
            Err(Box::new(err))
        }
    }
}
