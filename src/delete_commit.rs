// PACKAGES
use std::fs;
use serde::{Deserialize, Serialize};
use serde_yaml::{self};
use crate::structs::structs_mod::{FileChangeLog, CommitFiles, BranchChangesLog};
use crate::structs;
use crate::log;

fn verify_if_commit_exist(commit_to_verify: &str, branch:&BranchChangesLog) -> Result< usize, std::io::Error> {
    for i in 0..branch.commits_files.len(){
        if branch.commits_files[i].commit_hash == commit_to_verify.to_string(){
            return Ok(i);
        }
    }
    Err(std::io::Error::new(std::io::ErrorKind::NotFound, "Commit doesn't exist"))
}


fn verify_if_left_commits_use_the_diff(commit_hash_to_verify: &str, left: &Vec<CommitFiles>) -> bool {

    for commit_file in left{
        for file_change_log in &commit_file.files_changelogs{
            if file_change_log.hash_changelog==commit_hash_to_verify.to_string(){
                return true;
            }
        }
    }
    return false;

}

// Function to remove a file from the staging area
pub fn delete(commit_to_remove: &str) -> Result<(), Box<dyn std::error::Error>> {
    let path = "my_vcs/";
    let mut branch: structs::structs_mod::BranchChangesLog =
    structs::StructWriter::read_struct_from_file(&format!("{}{}", path, "branch_changes_log.yml"))?;

    let index = verify_if_commit_exist(commit_to_remove,&branch).unwrap();
    let commits_files = &branch.commits_files;

    //split the vector [hash_to_delet,last_hash] rest
    let (left, right) = commits_files.split_at(index);

    // delete diffs files
    for commit_file in right{
        for file_change_log in &commit_file.files_changelogs{
            if !verify_if_left_commits_use_the_diff(&file_change_log.hash_changelog.clone(), &left.to_vec()){
                let file_to_delete = format!("{}{}",file_change_log.hash_files_path, file_change_log.hash_changelog);
                fs::remove_file(file_to_delete)?;}
        }
    }

    branch.commits_files = left.to_vec();
    structs::StructWriter::update_struct_file(&format!("{}{}", path, "branch_changes_log.yml"), &branch)?;
    log::start(format!("delete {}", commit_to_remove));
    Ok(())

}
