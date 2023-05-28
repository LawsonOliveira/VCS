pub mod delete_commit_fn{
    // PACKAGES
    use std::fs;
    use serde::{Deserialize, Serialize};
    use serde_yaml::{self};
    use crate::structs::structs_mod::{FileChangeLog,BranchChangesLog};
    use crate::structs::{read_struct_from_file,update_struct_file};

    fn verify_if_commit_exist(commit_to_verify: &str, branch:&BranchChangesLog) -> Result< usize, std::io::Error> {
        for i in 0..branch.files_changelogs.len(){
            if branch.files_changelogs[i].hash_changelog.contains(&commit_to_verify.to_string()){
                return Ok(i);
            }
        }
        Err(std::io::Error::new(std::io::ErrorKind::NotFound, "Commit doesn't exist"))
    }

    fn delete_commit(commit_to_delete: &str, branch:&BranchChangesLog, file_index: usize) -> Result< usize, std::io::Error> {
        let file_to_delete_commit = &branch.files_changelogs[file_index];

        //get the position of the hash to delet
        let Some(position) = file_to_delete_commit.hash_changelog.iter().position(|x| x == commit_to_delete) else{
            panic!("Can't find commit: '{commit_to_delete}'");
        };


        //split the vector [hash_to_delet,last_hash] rest
        let (left, right) = file_to_delete_commit.hash_changelog.split_at(position);

        for hash in right{
            let file_to_delet = format!("{}{}",file_to_delete_commit.hash_files_path, hash);
            fs::remove_file(file_to_delet)?;
        }

        Ok(position)
    }

    fn remove_commit_info(branch: &mut BranchChangesLog, file_index: usize, commit_to_delete_position: usize) -> Result<(), std::io::Error> {
        let quantity_commits = branch.files_changelogs[file_index].hash_changelog.len() - 1;
        println!("Quantitycommits:{} delete_position:{}",quantity_commits,commit_to_delete_position);
        for i in commit_to_delete_position..=quantity_commits{
            branch.files_changelogs[file_index].hash_changelog.pop();
            println!("popei!")
        }
        Ok(())
    }

    // Function to remove a file from the staging area
    pub fn remove(commit_to_remove: &str) -> Result<(), std::io::Error> {
        let path = "my_vcs/branch_changes_log.yml";
        let mut branch: BranchChangesLog = read_struct_from_file(path).unwrap();

        let index = verify_if_commit_exist(commit_to_remove,&branch).unwrap();
        let commit_position:usize = delete_commit(commit_to_remove,&branch,index).unwrap();
        remove_commit_info(&mut branch,index,commit_position);

        update_struct_file(path, &branch);
        Ok(())
    }
}