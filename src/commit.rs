use std::fs;
use std::io;
use std::path::Path;

use std::io::{BufRead, BufReader, Read};
use std::collections::HashMap;

use diffy::{apply, Patch, PatchFormatter};
use sha2::{Digest, Sha256};

use serde_yaml;
use hex;
use crate::log;
use crate::structs;
use crate::remove;
use crate::add::calculate_file_hash;
use crate::structs::structs_mod::{Repository, Branch, Commit, FileChangeLog};



//// Falta trabalhar o caso inicial
pub fn create_commit(message: &str) -> Result<(), Box<dyn std::error::Error>> {
    let path = "my_vcs/";

    let mut repository: structs::structs_mod::Repository =
    structs::StructWriter::read_struct_from_file(&format!("{}{}", &path, "my_repo.yml"))?;

    // 1. Find the branch in the repository based on the given branch name

    let branch = match repository.branches.iter_mut().find(|b| b.branch_name == repository.current_branch) {
        Some(branch) => branch,
        None => create_branch(&mut repository),
    };

    // 2. Prepare the changes to be committed using your existing "add_to_version_control" function
    let changes_to_commit = prepare_changes_to_commit(path.to_string())?;

    // 3. Create a new Commit struct to represent the commit being made
    let mut commit = Commit {
        files_changelogs: Vec::new(),
        commit_hash: String::new(),
        parent_commits: Vec::new(),
    };

    // 6. Generate a unique commit hash for identification purposes
    let commit_hash = calculate_file_hash(&"my_vcs/log.yml")?;

    
    // 4. Set the parent commits for the new commit
    if let Some(parent_commit_hash) = branch.commits.last().map(|c| c.commit_hash.clone()) {
        commit.parent_commits.push(parent_commit_hash);
    }
    else{
        commit.parent_commits.push(commit_hash.clone());
    }
    


    // 5. Process the changes to be committed and create FileChangeLog structs for each change
    for (filename, filehash) in changes_to_commit {
        let first_version_file_changelog = true;
        if !has_file_change_log_with_original_file(branch, &filename) {
            let new_file_change_log = create_file_change_log_to_first_commit(filename, path.to_string(), filehash)?;
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

    structs::StructWriter::update_struct_file(&format!("{}{}", path, "my_repo.yml"), &repository)?;
    // 9. Return the commit hash
    Ok(())
}



fn create_branch(repository: &mut structs::structs_mod::Repository) -> &mut structs::structs_mod::Branch {
    let new_branch = structs::structs_mod::Branch {
        branch_name: repository.current_branch.clone(),
        commits: Vec::new(),
        head_commit_hash: String::new(),
    };
    repository.branches.push(new_branch);
    repository.branches.last_mut().unwrap()
}


fn has_file_change_log_with_original_file(branch: &Branch, original_file: &str) -> bool {
    for commit in &branch.commits {
        for file_change_log in &commit.files_changelogs {
            if file_change_log.original_file == original_file {
                return true;
            }
        }
    }
    false
}



fn prepare_changes_to_commit(path: String) -> Result<HashMap<String, String>, std::io::Error> {
    let staging_area_path = format!("{}/staging_area.yml", path);
    let mut file_changes: HashMap<String, String> = HashMap::new();
    let lines = read_file_lines(&staging_area_path)?;
    for line in lines {
        let binding = line.to_string();
        let splited_file_name_and_hash: Vec<&str> = binding.split(":").collect();
        let filename_to_commit = splited_file_name_and_hash[0].trim().to_string();
        let filehash_to_commit = splited_file_name_and_hash[1].trim().to_string();
        file_changes.insert(filename_to_commit, filehash_to_commit);
    }
    Ok(file_changes)
}




fn create_file_change_log_to_first_commit(filename_to_commit: String, path: String, filehash_to_commit: String)  -> Result<FileChangeLog, std::io::Error> {

    let new_file_change_log = FileChangeLog {
        original_file_path: String::from("./"),
        original_file: filename_to_commit.clone(),
        last_file: filename_to_commit.clone(),
        last_file_path: String::from("./"),
        hash_changelog: filehash_to_commit.clone(),
        hash_files_path: format!("{}{}", &path, "saves/"),
        version: 0,
        parent_versions: vec![0; 1],
    };


    let filename_save_path = format!("{}{}", new_file_change_log.hash_files_path, new_file_change_log.hash_changelog);
    let content_to_commit = fs::read_to_string(format!("{}add_contents/{}.yml", path, filename_to_commit))?;

    let file = fs::File::create(filename_save_path)?;
    let patch = diffy::create_patch("", &content_to_commit);
    PatchFormatter::new().write_patch_into(&patch, &file);

    // Remove the last version of the file
    let use_log = false;
    remove::remove(&filename_to_commit, use_log);

    Ok((new_file_change_log))
}



fn create_file_change_log(filename: String, path: String, filehash: String, branch: &mut Branch) -> Result<FileChangeLog, std::io::Error> {
    let filepath = format!("{}add_contents/", path);
    let commit_tree: Vec<Commit> = build_commit_tree(branch, &branch.head_commit_hash)?;
    let files_changelogs_tree: Vec<FileChangeLog> = build_file_change_log_tree(&filename, &commit_tree)?;
    let file_parent_version_tree: Vec<FileChangeLog> = build_file_parent_version_tree(&files_changelogs_tree)?;

    let last_file = filename.clone();
    let last_file_path = filepath.clone();
    let hash_changelog = filehash.clone();
    let version = file_parent_version_tree.len();
    let parent_versions = vec![file_parent_version_tree.last().ok_or_else(|| io::Error::new(io::ErrorKind::NotFound, "File parent version tree is empty"))?.version];


    let new_file_change_log = FileChangeLog {
        original_file_path: file_parent_version_tree.last().ok_or_else(|| io::Error::new(io::ErrorKind::NotFound, "File parent version tree is empty"))?.original_file_path.clone(),
        original_file: file_parent_version_tree.last().ok_or_else(|| io::Error::new(io::ErrorKind::NotFound, "File parent version tree is empty"))?.original_file.clone(),
        last_file,
        last_file_path,
        hash_changelog,
        hash_files_path: file_parent_version_tree.last().ok_or_else(|| io::Error::new(io::ErrorKind::NotFound, "File parent version tree is empty"))?.hash_files_path.clone(),
        version: version.try_into().unwrap(),
        parent_versions,
    };

    let mut previous_version: String = String::new();
    for file_changelog_version in file_parent_version_tree{

        let diff_path = format!("{}{}",file_changelog_version.hash_files_path,file_changelog_version.hash_changelog);
        let diff_content = fs::read_to_string(&diff_path).expect("could not read hash file");
        let patch: Patch<str> = Patch::from_str(&diff_content).unwrap();
        let previous_version = apply(&previous_version.to_owned(), &patch).unwrap();
    }


    // Generate the diff between the previous version and the content to commit
    let filename_save_path = format!("{}{}", new_file_change_log.hash_files_path, new_file_change_log.hash_changelog);
    let content_to_commit = fs::read_to_string(format!("{}{}.yml", filepath, filename))?;

    let file = fs::File::create(filename_save_path)?;
    let patch = diffy::create_patch(&previous_version, &content_to_commit);
    PatchFormatter::new().write_patch_into(&patch, &file);


    // Remove the last version of the file
    let use_log = false;
    remove::remove(&filename, use_log);

    Ok(new_file_change_log)
}




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



fn build_file_change_log_tree(original_file: &str, commit_tree: &[Commit]) -> Result<Vec<FileChangeLog>, io::Error> {
    let mut file_change_log_tree: Vec<FileChangeLog> = Vec::new();
    let mut previous_name = original_file.to_string();


    for commit in commit_tree.iter() {
        for file_change_log in commit.files_changelogs.iter() {
            if file_change_log.original_file == previous_name {
                previous_name = file_change_log.last_file.clone();
                file_change_log_tree.push(file_change_log.clone());
            }
        }
    }

    Ok(file_change_log_tree)
}

fn build_file_parent_version_tree(file_change_log_tree: &[FileChangeLog]) -> Result<Vec<FileChangeLog>, io::Error> {
    let mut file_parent_version_tree: Vec<FileChangeLog> = file_change_log_tree.to_vec();
    file_parent_version_tree.reverse();

    let initial_version = file_change_log_tree
        .last()
        .ok_or_else(|| io::Error::new(io::ErrorKind::NotFound, "File change log tree is empty"))?
        .version;

    let mut current_version = initial_version;

    while let Some(parent_version) = file_change_log_tree
        .iter()
        .find(|file_change_log| file_change_log.version == current_version)
        .and_then(|file_change_log| file_change_log.parent_versions.first())
        .cloned()
    {
        if parent_version == 0 {
            break;
        }

        if let Some(parent_file_change_log) = file_change_log_tree
            .iter()
            .find(|file_change_log| file_change_log.version == parent_version)
        {
            file_parent_version_tree.push(parent_file_change_log.clone());
            current_version = parent_version;
        } else {
            return Err(io::Error::new(io::ErrorKind::NotFound, "Parent file change log not found"));
        }
    }

    file_parent_version_tree.reverse();
    Ok(file_parent_version_tree)
}


fn read_file_lines(path: &str) -> Result<Vec<String>, io::Error> {
    let file = fs::File::open(path)?;
    let reader = BufReader::new(file);
    let lines: Vec<String> = reader.lines().collect::<Result<_, _>>()?;
    Ok(lines)
}