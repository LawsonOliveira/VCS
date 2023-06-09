use std::io;

use crate::structs;
use crate::log;
use crate::create_branch::verify_if_branch_exists;


/// Deletes a branch with the given name.
/// Returns `Ok(())` if the branch is successfully deleted, or an `Err` with an `io::ErrorKind::NotFound` if the branch does not exist.
pub fn delete_branch(branch_name: &str) -> Result<(), Box<dyn std::error::Error>> {
    let path = "my_vcs/";

    // Verify if the branch exists
    if !verify_if_branch_exists(branch_name)? {
        return Err(Box::new(io::Error::new(io::ErrorKind::NotFound, "Branch does not exist")));
    }

    let mut repository: structs::structs_mod::Repository =
        structs::StructWriter::read_struct_from_file(&format!("{}{}", &path, "my_repo.yml")).unwrap();

    if let Some(index) = repository.branches.iter().position(|b| b.branch_name == branch_name) {
        let branch = repository.branches.remove(index);

        // Delete file_changeslog and associated files
        for commit in branch.commits {
            for file_change_log in commit.files_changelogs {
                if !is_file_used_by_other_branch(&branch_name, &file_change_log.hash_changelog, &repository)? {
                    // Delete the file with hash_changelog
                    let file_path = format!("{}saves/{}", path, file_change_log.hash_changelog);
                    std::fs::remove_file(file_path)?;
                }
            }
        }
    }
    repository.current_branch = "main".to_string(); // Set the current branch to "main"
    structs::StructWriter::update_struct_file(&format!("{}{}", path, "my_repo.yml"), &repository)?;
    println!("Branch '{}' was deleted", branch_name);
    log::start(format!("delete_branch {}", &branch_name));

    Ok(())
}


/// Checks if a file with the given hash is used by any branch other than the specified branch.
/// Returns `Ok(true)` if the file is used by another branch, `Ok(false)` if it is not, or an `Err` if there was an error.
fn is_file_used_by_other_branch(branch_name: &str, file_hash: &str, repository: &structs::structs_mod::Repository) -> Result<bool, Box<dyn std::error::Error>> {
    let result = repository.branches.iter().any(|branch| {
        branch.branch_name != branch_name && branch.commits.iter().any(|commit| {
            commit.files_changelogs.iter().any(|f| f.hash_changelog == file_hash)
        })
    });

    Ok(result)
}


