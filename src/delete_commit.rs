use std::{fs, io};
use std::error::Error;
use std::collections::HashSet;

use crate::structs::structs_mod::{FileChangeLog, Branch, Commit};
use crate::structs;
use crate::log;
use crate::commit::{build_commit_tree};
use crate::delete_branch::is_file_used_by_other_branch;

/// Verifies if a commit exists in a branch.
/// Returns `Ok(true)` if the commit exists, otherwise returns an `Err` with an `io::ErrorKind::NotFound`.
pub fn verify_if_commit_exist(commit_to_verify: &str, branch: &Branch) -> Result<bool, io::Error> {
    for commit in branch.commits.iter() {
        if commit.commit_hash == commit_to_verify {
            return Ok(true);
        }
    }
    Err(io::Error::new(io::ErrorKind::NotFound, "Commit doesn't exist"))
}

/// Deletes files from a folder based on the commits to delete.
/// Returns `Ok(())` if the files are deleted successfully, otherwise returns an `Err` with the corresponding error.
fn delete_diff_files(folder_path: &str, commits_to_delete: &[Commit], repository: &structs::structs_mod::Repository) -> Result<(), Box<dyn Error>> {
    let folder_entries = fs::read_dir(folder_path)?;
    let files_changelogs_to_delete: Vec<FileChangeLog> = commits_to_delete
        .iter()
        .flat_map(|commit| commit.files_changelogs.clone())
        .collect();

    let filenames_to_delete: Vec<String> = files_changelogs_to_delete
        .iter()
        .map(|hash_changelog| hash_changelog.hash_changelog.clone())
        .collect();

    for entry in folder_entries {
        let entry = entry?;
        let file_name = entry.file_name();
        let file_name_str = file_name.to_string_lossy().into_owned();
        if !is_file_used_by_other_branch(&repository.current_branch, &file_name_str, &repository)?{
            if filenames_to_delete.contains(&file_name_str) {
                let file_path = entry.path();
                if file_path.is_file() {
                    fs::remove_file(file_path)?;
                    println!("Deleted file: {}", file_name_str);
                }
            }
        }
    }

    Ok(())
}



/// Deletes the child commits from a branch.
/// Returns `Ok(())` if the commits are deleted successfully, otherwise returns an `Err` with an `io::ErrorKind::NotFound`.
fn delete_child_commits(branch: &mut Branch, commits_to_delete: &[Commit]) -> Result<(), io::Error> {
    let final_commits_count = branch.commits.len() - commits_to_delete.len();
    branch.commits.retain(|commit| !commits_to_delete
        .iter()
        .any(|delete_commit| delete_commit.commit_hash == commit.commit_hash));

    if branch.commits.len() != final_commits_count {
        return Err(io::Error::new(io::ErrorKind::NotFound, "Not at least one matching commit was found"));
    }

    Ok(())
}

/// Generates a list of commits to delete based on a given commit hash to delete.
/// Returns `Ok(Vec<Commit>)` with the list of commits to delete, or an `Err` with an `io::ErrorKind::NotFound` if no matching commits are found.
fn gen_commits_to_delete(branch: &Branch, commit_hash_to_delete: &str) -> Result<Vec<Commit>, io::Error> {
    let mut commit_hash_parent: HashSet<String> = HashSet::new();
    commit_hash_parent.insert(commit_hash_to_delete.to_string());
    let mut commits_to_delete: HashSet<String> = HashSet::new();

    let mut processed_commits: HashSet<String> = HashSet::new(); // Track processed commits

    if let Some(commit) = branch.commits.iter().find(|commit| commit.commit_hash == commit_hash_to_delete) {
        commits_to_delete.insert(commit.commit_hash.clone());
        processed_commits.insert(commit.commit_hash.clone()); // Mark the commit as processed
    }
    
    // Process commits while there are still commits to consider
    while !commits_to_delete.is_empty() {
        let current_commit_hash = commits_to_delete.iter().next().unwrap().clone(); // Get the next commit to consider
        commits_to_delete.remove(&current_commit_hash); // Remove the commit from the set
        
        for commit in &branch.commits {
            if commit.parent_commits.iter().any(|parent| parent == &current_commit_hash) {
                commit_hash_parent.extend(commit.parent_commits.clone());
                if !processed_commits.contains(&commit.commit_hash) {
                    commits_to_delete.insert(commit.commit_hash.clone());
                    processed_commits.insert(commit.commit_hash.clone()); // Mark the commit as processed
                    println!("{:?}", commit.commit_hash.clone());
                }
            }
        }
    }

    if processed_commits.is_empty() {
        return Err(io::Error::new(io::ErrorKind::NotFound, "No matching commits found"));
    }

    let commits_to_delete_vec: Vec<Commit> = branch.commits.iter()
        .filter(|commit| processed_commits.contains(&commit.commit_hash))
        .cloned()
        .collect();

    Ok(commits_to_delete_vec)
}



/// Deletes a commit from the repository.
/// Returns `Ok(())` if the commit is deleted successfully, otherwise returns an `Err` with the corresponding error.
pub fn delete(commit_to_remove: &str) -> Result<(), Box<dyn Error>> {
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

    // 2. Verify if the commit to remove exists in the branch
    if !verify_if_commit_exist(commit_to_remove, &branch)? {
        return Err(Box::new(io::Error::new(io::ErrorKind::NotFound, "Commit not found")));
    }

    // 3. Generate the list of commits to delete
    let commits_to_delete: Vec<Commit> = gen_commits_to_delete(&branch, &commit_to_remove)?;

    let new_head_commit_hash: String = commits_to_delete[0].parent_commits[0].clone();

    // 4. Delete diff files not in saves_to_retain
    let mut repository_copy: structs::structs_mod::Repository =
    structs::StructWriter::read_struct_from_file(&format!("{}{}", &path, "my_repo.yml"))?;
    delete_diff_files(&format!("{}saves/", path), &commits_to_delete, &repository_copy)?;

    // 5. Delete the commits from the branch
    delete_child_commits(branch, &commits_to_delete)?;

    // 6. Update the repository and log the deletion
    branch.head_commit_hash = new_head_commit_hash;
    structs::StructWriter::update_struct_file(&format!("{}{}", path, "my_repo.yml"), &repository)?;

    println!("Commit with hash {} deleted", &commit_to_remove);
    log::start(format!("delete_commit {}", &commit_to_remove));
    Ok(())
}
