use std::io::{self, Write};

use crate::structs;
use crate::log;


/// Verify if a branch with the given name already exists.
/// Returns `Ok(true)` if the branch exists, otherwise returns `Ok(false)`.
/// Returns an `Err` with the corresponding error if there's an error reading the repository file.
pub fn verify_if_branch_exists(branch_name: &str) -> Result<bool, Box<dyn std::error::Error>> {
    let path = "my_vcs/";
    let repository: structs::structs_mod::Repository =
        structs::StructWriter::read_struct_from_file(&format!("{}{}", &path, "my_repo.yml"))?;
    Ok(repository.branches.iter().any(|branch| branch.branch_name == branch_name))
}

/// Create a new branch with the given name.
/// Returns `Ok(())` if the branch was created successfully, otherwise returns an `Err` with the corresponding error.
pub fn create_branch(branch_name: &str) -> Result<(), Box<dyn std::error::Error>> {
    let path = "my_vcs/";

    // Verify if the branch already exists
    if verify_if_branch_exists(branch_name)? {
        return Err(Box::new(io::Error::new(io::ErrorKind::NotFound, "Branch already exists")));
    }

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
    log::start(format!("create_branch {}", &branch_name));
    Ok(())
}
