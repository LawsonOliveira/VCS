use std::fs;
use serde::{Deserialize, Serialize};
use serde_yaml::{self};
use std::io;
use std::io::Write;
use crate::structs::structs_mod::{FileChangeLog, Branch, Commit};
use crate::structs;
use crate::log;
use diffy::{apply, Patch, PatchFormatter};
use crate::commit::{build_commit_tree, build_file_change_log_tree, build_file_parent_version_tree};
use std::fs::File;

/// Verifies if a commit exists in a branch.
///
/// This function checks if a commit with the specified commit hash exists in the given branch.
///
/// # Arguments
///
/// * `commit_to_verify` - The commit hash to verify.
/// * `branch` - The branch to search for the commit in.
///
/// # Returns
///
/// * `Ok(true)` if the commit exists.
/// * `Err` with `std::io::Error` of kind `NotFound` if the commit doesn't exist.
fn verify_if_commit_exist(commit_to_verify: &str, branch: &Branch) -> Result<bool, std::io::Error> {
    for commit in branch.commits.iter() {
        if commit.commit_hash == commit_to_verify {
            return Ok(true);
        }
    }
    Err(std::io::Error::new(std::io::ErrorKind::NotFound, "Commit doesn't exist"))
}

/// Changes the version of files to a specific commit.
///
/// This function changes the version of files in the staging area to the version
/// specified by the given commit hash. It reads the commit tree, file change log tree,
/// and file parent version tree to determine the changes to be made. The previous versions
/// of the files are applied to create the desired version.
///
/// # Arguments
///
/// * `commit_target` - The commit hash of the target version.
///
/// # Returns
///
/// * `Ok(())` if the version change is successful.
/// * `Err` with `Box<dyn std::error::Error>` if an error occurs during the version change process.
pub fn change_version(commit_target: &str) -> Result<(), Box<dyn std::error::Error>> {
    let path = "my_vcs/";
    let mut repository: structs::structs_mod::Repository =
        structs::StructWriter::read_struct_from_file(&format!("{}{}", &path, "my_repo.yml"))?;

    // Find the branch in the repository based on the current branch name
    let branch = match repository.branches.iter_mut().find(|b| b.branch_name == repository.current_branch) {
        Some(branch) => branch,
        None => {
            println!("Branch not found");
            return Err(Box::new(io::Error::new(io::ErrorKind::NotFound, "Branch not found")));
        }
    };

    match verify_if_commit_exist(commit_target, &branch) {
        Ok(index) => {
            // Build commit tree for the given branch and commit hash
            let commit_tree: Vec<Commit> = build_commit_tree(branch, commit_target)?;

            // Iterate through the commit tree and update file versions
            for commit in &commit_tree {
                for file_change_log in &commit.files_changelogs {
                    let filename = file_change_log.last_file.clone();

                    // Build file change log tree and file parent version tree
                    let files_changelogs_tree: Vec<FileChangeLog> = build_file_change_log_tree(&filename, &commit_tree)?;
                    let file_parent_version_tree: Vec<FileChangeLog> = build_file_parent_version_tree(&files_changelogs_tree)?;

                    let mut previous_version: String = String::new();
                    let last_file_version: &FileChangeLog = file_parent_version_tree.last().unwrap();

                    // Apply previous versions to create the desired version
                    for file_changelog_version in &file_parent_version_tree {
                        let diff_path = format!("{}{}", file_changelog_version.hash_files_path, file_changelog_version.hash_changelog);
                        let diff_content = fs::read_to_string(&diff_path).expect("could not read hash file");
                        let patch: Patch<str> = Patch::from_str(&diff_content).unwrap();
                        previous_version = apply(&previous_version.to_owned(), &patch).unwrap();
                    }

                    let filename_save_path = format!("{}{}", last_file_version.last_file_path, last_file_version.last_file);

                    // Open the file and write the previous version
                    let mut f = std::fs::OpenOptions::new().write(true).truncate(true).open(filename_save_path)?;
                    f.write(previous_version.as_bytes())?;
                    f.flush()?;
                }
            }

            // Update the head commit hash of the branch to the hash of the newly created commit
            branch.head_commit_hash = commit_target.to_string();

            structs::StructWriter::update_struct_file(&format!("{}{}", path, "my_repo.yml"), &repository)?;

            println!("Moved to {}", &commit_target);
            log::start(format!("Moved to:{} ", &commit_target));

            Ok(())
        }
        Err(err) => {
            // Handle the error case where the commit doesn't exist
            println!("Error commit does not exist: {}", err);
            Err(Box::new(err))
        }
    }
}
