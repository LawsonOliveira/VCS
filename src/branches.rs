use std::fs;
use std::io;
use std::io::Write;
use std::fs::File;
use serde::{Deserialize, Serialize};
use serde_yaml::{self};
use crate::structs::structs_mod::{FileChangeLog, Branch, Commit};
use crate::structs;
use crate::log;
use diffy::{apply, Patch, PatchFormatter};
use crate::commit::{build_commit_tree, build_file_change_log_tree, build_file_parent_version_tree};

/// Verifies if a branch exists in the repository.
///
/// This function checks if the given branch name exists in the repository.
///
/// # Arguments
///
/// * `branch_name` - The name of the branch to verify.
///
/// # Returns
///
/// * `true` if the branch exists, `false` otherwise.
fn verify_if_branch_exist(branch_name: &str) -> bool {
    let path = "my_vcs/";
    let repository: structs::structs_mod::Repository =
        structs::StructWriter::read_struct_from_file(&format!("{}{}", &path, "my_repo.yml")).unwrap();
    repository.branches.iter().any(|branch| branch.branch_name == branch_name)
}

/// Creates a new branch in the repository.
///
/// This function creates a new branch with the given branch name in the repository.
/// If the branch already exists, an error is returned.
///
/// # Arguments
///
/// * `branch_name` - The name of the branch to create.
///
/// # Returns
///
/// * `Ok(())` if the branch is created successfully.
/// * `Err` if the branch already exists.
pub fn create_branch(branch_name: &str) -> Result<(), Box<dyn std::error::Error>> {
    let path = "my_vcs/";
    if !verify_if_branch_exist(branch_name) {
        let mut repository: structs::structs_mod::Repository =
            structs::StructWriter::read_struct_from_file(&format!("{}{}", &path, "my_repo.yml")).unwrap();

        let previous_branch = match repository.branches.iter_mut().find(|b| b.branch_name == repository.current_branch) {
            Some(previous_branch) => {
                let new_branch = structs::structs_mod::Branch {
                    branch_name: branch_name.to_string(),
                    head_commit_hash: previous_branch.head_commit_hash.clone(),
                    commits: previous_branch.commits.clone(),
                };
                repository.branches.push(new_branch);
                structs::StructWriter::update_struct_file(&format!("{}{}", path, "my_repo.yml"), &repository)?;
            }
            None => {
                let new_branch = structs::structs_mod::Branch {
                    branch_name: branch_name.to_string(),
                    head_commit_hash: String::new(),
                    commits: Vec::new(),
                };
                repository.branches.push(new_branch);
                structs::StructWriter::update_struct_file(&format!("{}{}", path, "my_repo.yml"), &repository)?;
            }
        };
        println!("Branch '{}' was created", branch_name);
        Ok(())
    } else {
        Err(Box::new(std::io::Error::new(std::io::ErrorKind::AlreadyExists, "Branch already exists")))
    }
}

/// Changes the current branch in the repository.
///
/// This function changes the current branch to the branch with the given name.
/// If the branch does not exist, an error is returned.
///
/// # Arguments
///
/// * `branch_name` - The name of the branch to change to.
///
/// # Returns
///
/// * `Ok(())` if the branch is changed successfully.
/// * `Err` if the branch does not exist.
pub fn change_branch(branch_name: &str) -> Result<(), Box<dyn std::error::Error>> {
    let path = "my_vcs/";
    if !verify_if_branch_exist(branch_name) {
        println!("Error: Branch '{}' does not exist.", branch_name);
        return Err(Box::new(std::io::Error::new(std::io::ErrorKind::NotFound, "Branch does not exist")));
    } else {
        let mut repository: structs::structs_mod::Repository =
            structs::StructWriter::read_struct_from_file(&format!("{}{}", &path, "my_repo.yml")).unwrap();
        repository.current_branch = branch_name.to_string();
        structs::StructWriter::update_struct_file(&format!("{}{}", path, "my_repo.yml"), &repository)?;
        println!("Now you are working in the branch '{}'", branch_name);

        Ok(())
    }
}




/// Deletes a branch from the repository.
///
/// This function deletes the branch with the given name from the repository.
/// If the branch does not exist, an error is returned.
///
/// # Arguments
///
/// * `branch_name` - The name of the branch to delete.
///
/// # Returns
///
/// * `Ok(())` if the branch is deleted successfully.
/// * `Err` if the branch does not exist.

pub fn delete_branch(branch_name: &str) -> Result<(), Box<dyn std::error::Error>> {
    let path = "my_vcs/";
    let mut repository: structs::structs_mod::Repository =
        structs::StructWriter::read_struct_from_file(&format!("{}{}", &path, "my_repo.yml")).unwrap();

    if let Some(index) = repository.branches.iter().position(|b| b.branch_name == branch_name) {
        let branch = repository.branches.remove(index);

        // Delete file_changeslog and associated files
        for commit in branch.commits {
            for file_change_log in commit.files_changelogs {
                if !is_file_used_by_other_branch(&file_change_log.hash_changelog, &repository) {
                    // Delete the file with hash_changelog
                    let file_path = format!("{}saves/{}", path, file_change_log.hash_changelog);

                    std::fs::remove_file(file_path)?;

                    // Remove the file_change_log from repository.commits
                    for other_branch in &mut repository.branches {
                        for other_commit in &mut other_branch.commits {
                            other_commit.files_changelogs.retain(|f| f.hash_changelog != file_change_log.hash_changelog);
                        }
                    }
                }
            }
        }

        repository.current_branch = "main".to_string(); // Set the current branch to "main"
        structs::StructWriter::update_struct_file(&format!("{}{}", path, "my_repo.yml"), &repository)?;
        println!("Branch '{}' was deleted", branch_name);
        Ok(())
    } else {
        Err(Box::new(std::io::Error::new(std::io::ErrorKind::NotFound, "Branch not found")))
    }
}
/// Checks if a file is used by other branches in the repository.
///
/// This function checks if a file with the given hash is used by other branches
/// in the repository, excluding the current branch.
///
/// # Arguments
///
/// * `file_hash` - The hash of the file to check.
/// * `repository` - The repository structure.
///
/// # Returns
///
/// * `true` if the file is used by other branches, `false` otherwise.
fn is_file_used_by_other_branch(file_hash: &str, repository: &structs::structs_mod::Repository) -> bool {
    for branch in &repository.branches {
        if branch.branch_name != repository.current_branch {
            for commit in &branch.commits {
                if commit.files_changelogs.iter().any(|f| f.hash_changelog == file_hash) {
                    return true;
                }
            }
        }
    }
    false
}

