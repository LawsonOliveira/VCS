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

fn verify_if_branch_exist(branch_name: &str) -> bool {
    let path = "my_vcs/";
    let repository: structs::structs_mod::Repository =
        structs::StructWriter::read_struct_from_file(&format!("{}{}", &path, "my_repo.yml")).unwrap();
    repository.branches.iter().any(|branch| branch.branch_name == branch_name)
}
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