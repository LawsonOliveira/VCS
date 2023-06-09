use std::fs;
use std::path::Path;
use std::io::{self, BufRead, BufReader, Read};
use std::collections::HashMap;
use diffy::{apply, Patch, PatchFormatter};

use crate::log;
use crate::structs;
use crate::remove;
use crate::add::calculate_file_hash;
use crate::structs::structs_mod::{Repository, Branch, Commit, FileChangeLog};


// Function to create a new commit
/// Returns `Ok(())` if the commit was created successfully, otherwise returns an `Err` with the corresponding error.
pub fn create_commit(message: &str) -> Result<(), Box<dyn std::error::Error>> {
    let path = "my_vcs/";

    let mut repository: structs::structs_mod::Repository =
    structs::StructWriter::read_struct_from_file(&format!("{}{}", &path, "my_repo.yml"))?;

    // 1. Find the branch in the repository based on the given branch name
    let branch = repository
        .branches
        .iter_mut()
        .find(|b| b.branch_name == repository.current_branch)
        .ok_or("Branch not found")?;

    // 2. Prepare the changes to be committed using your existing "add_to_version_control" function
    let changes_to_commit = prepare_changes_to_commit(path.to_string())?;

    // 3. Create a new Commit struct to represent the commit being made
    let mut commit = Commit {
        files_changelogs: Vec::new(),
        commit_hash: String::new(),
        parent_commits: Vec::new(),
    };

    // 4. Generate a unique commit hash for identification purposes
    let commit_hash = calculate_file_hash(&"my_vcs/log.yml")?;
    
    // 5. Set the parent commits for the new commit
    if &branch.head_commit_hash != "" {
        commit.parent_commits.push(branch.head_commit_hash.clone());
    }
    else{
        commit.parent_commits.push(commit_hash.clone());
    }
    

    // 6. Process the changes to be committed and create FileChangeLog structs for each change
    for (filename, filehash) in changes_to_commit {
        let first_version_file_changelog = true;
        if !has_file_change_log_with_original_file(branch, &filename)? {
            let new_file_change_log = create_file_change_log_to_first_commit(&filename, &path.to_string(), &filehash)?;
            commit.files_changelogs.push(new_file_change_log);
        }
        else{
            let new_file_change_log = create_file_change_log(filename, path.to_string(), filehash, branch)?;
            commit.files_changelogs.push(new_file_change_log);
        }
        

    }            


    // 7. Update the commit hash and add the new commit to the branch
    commit.commit_hash = commit_hash.clone();
    branch.commits.push(commit);

    // 8. Update the head commit hash of the branch to the hash of the newly created commit
    branch.head_commit_hash = commit_hash.clone();

    log::start(format!("commit \n message: {} \n commit hash: {}", &message,&commit_hash));

    structs::StructWriter::update_struct_file(&format!("{}{}", path, "my_repo.yml"), &repository)?;
    Ok(())
}





// Check if a branch has a file change log with the original file
/// Returns `Ok(true)` if exists at least one file_change_log for the file to commit, otherwise returns `Ok(false)`.
/// Otherwise returns an `Err` with the corresponding error.
fn has_file_change_log_with_original_file(branch: &Branch, original_file: &str) -> Result<bool, &'static str> {
    for commit in &branch.commits {
        for file_change_log in &commit.files_changelogs {
            if file_change_log.original_file == original_file {
                return Ok(true);
            }
        }
    }
    Ok(false)
}


/// Prepares changes to commit by reading a staging area file.
/// Returns a `Result` indicating either a `HashMap<String, String>` containing the file changes
/// or an `std::io::Error` if there was an error reading the file.
fn prepare_changes_to_commit(path: String) -> Result<HashMap<String, String>, std::io::Error> {
    let staging_area_path = format!("{}/staging_area.yml", path);
    let mut file_changeslog: HashMap<String, String> = HashMap::new();

    let file = fs::File::open(path)?;
    let reader = BufReader::new(file);
    let lines: Vec<String> = reader.lines().collect::<Result<_, _>>()?;

    for line in lines {
        let binding = line.to_string();
        let splited_file_name_and_hash: Vec<&str> = binding.split(":").collect();
        let filename_to_commit = splited_file_name_and_hash[0].trim().to_string();
        let filehash_to_commit = splited_file_name_and_hash[1].trim().to_string();
        file_changeslog.insert(filename_to_commit, filehash_to_commit);
    }
    Ok(file_changeslog)
}



// Create a file change log for the first commit of a file
/// Returns a `Result` indicating either a `FileChangeLog` struct containing the file change log information
/// or an `std::io::Error` if there was an error during file operations.
fn create_file_change_log_to_first_commit(filename_to_commit: &String, path: &String, filehash_to_commit: &String)  -> Result<FileChangeLog, std::io::Error> {

    let new_file_change_log = FileChangeLog {
        original_file_path: String::from("./"),
        original_file: filename_to_commit.clone(),
        last_file: filename_to_commit.clone(),
        last_file_path: String::from("./"),
        hash_changelog: filehash_to_commit.clone(),
        hash_files_path: format!("{}{}", &path, "saves/"),
        version: 0,
        parent_version: 0,
    };


    let filename_save_path = format!("{}{}", new_file_change_log.hash_files_path, new_file_change_log.hash_changelog);
    let content_to_commit = fs::read_to_string(format!("{}add_contents/{}.yml", &path, filename_to_commit))?;

    let file = fs::File::create(filename_save_path)?;
    let patch = diffy::create_patch("", &content_to_commit);
    PatchFormatter::new().write_patch_into(&patch, &file);

    // Remove the last version of the file
    let use_log = false;
    remove::remove(&filename_to_commit, use_log);

    Ok(new_file_change_log)
}



/// Create a file change log for a commit other than the first commit.
/// Returns a `Result` indicating either a `FileChangeLog` struct containing the file change log information
/// or an `std::io::Error` if there was an error during file operations.
fn create_file_change_log(filename: String, path: String, filehash: String, branch: &mut Branch) -> Result<FileChangeLog, std::io::Error> {
    let filepath = format!("{}add_contents/", path);
    let commit_tree: Vec<Commit> = build_commit_tree(branch, &branch.head_commit_hash)?;
    let file_changelogs_tree: Vec<FileChangeLog> = build_file_change_log_tree(&filename, &commit_tree)?;

    let last_file = filename.clone();
    let last_file_path = "./".to_string();
    let hash_changelog = filehash;
    let version: u32 = branch.commits.len() as u32;

    let parent = file_changelogs_tree.last().unwrap();


    let new_file_change_log = FileChangeLog {
        original_file_path: parent.original_file_path.clone(),
        original_file: parent.original_file.clone(),
        last_file,
        last_file_path,
        hash_changelog,
        hash_files_path: parent.hash_files_path.clone(),
        version: version,
        parent_version: parent.version.clone(),
    };

    let mut previous_versions: String = String::new();

    for file_changelog_version in file_changelogs_tree{

        let diff_path = format!("{}{}",file_changelog_version.hash_files_path,file_changelog_version.hash_changelog);
        let diff_content = fs::read_to_string(&diff_path).expect("could not read hash file");
        let patch: Patch<str> = Patch::from_str(&diff_content).unwrap();
        previous_versions = apply(&previous_versions.to_owned(), &patch).unwrap();
    }


    // Generate the diff between the previous version and the content to commit
    let filename_save_path = format!("{}{}", new_file_change_log.hash_files_path, new_file_change_log.hash_changelog);
    let content_to_commit = fs::read_to_string(format!("{}{}.yml", filepath, filename))?;

    let file = fs::File::create(filename_save_path)?;
    let patch = diffy::create_patch(&previous_versions, &content_to_commit);
    PatchFormatter::new().write_patch_into(&patch, &file);


    // Remove the last version of the file
    let use_log = false;
    remove::remove(&filename, use_log);

    Ok(new_file_change_log)
}


/// Build a commit tree for a branch and the commit_hash desired.
/// Returns a `Result` indicating either a vector of `Commit` structs representing the commit tree
/// or an `std::io::Error` if there was an error during the process.
pub fn build_commit_tree(branch: &Branch, head_commit_hash: &str) -> Result<Vec<Commit>, io::Error> {
    let mut commit_stack: Vec<String> = vec![head_commit_hash.to_owned()];
    let mut commit_tree: Vec<Commit> = Vec::new();

    while let Some(current_commit_hash) = commit_stack.pop() {
        if let Some(commit) = branch.commits.iter().find(|commit| (commit.commit_hash == current_commit_hash) && (commit.commit_hash != branch.commits[0].commit_hash)) {
            commit_tree.push(commit.clone());

            for parent_hash in commit.parent_commits.iter() {
                commit_stack.push(parent_hash.to_owned());
            }
        } 
    }
    commit_tree.push(branch.commits[0].clone());
    commit_tree.reverse();
    Ok(commit_tree)
}


/// Build a file_change_log tree based on the commit_tree and original_file_name in the head commit.
/// Returns a `Result` indicating either a vector of `FileChangeLog` structs representing the file change log tree
/// or an `std::io::Error` if there was an error during the process.
pub fn build_file_change_log_tree(original_file: &str, commit_tree: &[Commit]) -> Result<Vec<FileChangeLog>, io::Error> {
    let mut file_change_log_tree: Vec<FileChangeLog> = Vec::new();
    let mut previous_name = original_file.to_string();


    for commit in commit_tree.iter() {
        for file_change_log in commit.files_changelogs.iter() {
            if file_change_log.original_file == previous_name {
                previous_name = file_change_log.last_file.clone();
                file_change_log_tree.push(file_change_log.to_owned());
            }
        }
    }

    Ok(file_change_log_tree)
}

