use std::fs::{self, File};
use std::io::{self, Write};
use diffy::{apply, Patch, PatchFormatter};

use crate::structs::structs_mod::{FileChangeLog, Branch, Commit};
use crate::structs;
use crate::log;
use crate::commit::{build_commit_tree, build_file_change_log_tree};
use crate::delete_commit::verify_if_commit_exist;


/// Changes the version of files to a specific commit.
/// Returns `Ok(())` if the version was modified successfully, otherwise returns an `Err` with the corresponding error.
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
    
    // Verify if the commit to remove exists in the branch
    if !verify_if_commit_exist(commit_target, &branch)? {
        return Err(Box::new(io::Error::new(io::ErrorKind::NotFound, "Commit not found")));
    }

    // Build commit tree for the given branch and commit hash
    let commit_tree: Vec<Commit> = build_commit_tree(branch, commit_target)?;
    for commit in &commit_tree{
        for file_change_log in &commit.files_changelogs{
            let file_changelogs_tree: Vec<FileChangeLog> = build_file_change_log_tree(&file_change_log.last_file, &commit_tree)?;

            let mut previous_version: String = String::new();

            for file_changelog_version in &file_changelogs_tree {

                let diff_path = format!("{}{}", file_changelog_version.hash_files_path, file_changelog_version.hash_changelog);
                let diff_content = fs::read_to_string(&diff_path).expect("could not read hash file");
                let patch: Patch<str> = Patch::from_str(&diff_content).unwrap();
                previous_version =  apply(&previous_version.to_owned(), &patch)?;
            }

            let last_file_version: &FileChangeLog = file_changelogs_tree.last().unwrap();
            let filename_save_path = format!("{}{}", last_file_version.last_file_path, last_file_version.last_file);

            // Open the file and write the previous version
            let mut f = File::create(&filename_save_path)?;
            f.write_all(previous_version.as_bytes())?;
            f.flush()?;
        }

    }
    // Update the head commit hash of the branch to the hash of the newly created commit
    branch.head_commit_hash = commit_target.to_string();

    structs::StructWriter::update_struct_file(&format!("{}{}", path, "my_repo.yml"), &repository)?;

    println!("Moved to {}", &commit_target);
    log::start(format!("Change_version {} ", &commit_target));

    Ok(())
}
