pub mod delete_fn{
    // PACKAGES
    use std::fs;
    use serde::{Deserialize, Serialize};
    use serde_yaml::{self};
    use structs::structs_mod::ChangeLogs;
    use structs::structs_mod::BranchChangesLog;

    fn verify_if_file_exist(file_to_verify: &str, branch:BranchChangesLog) -> Result< i32, std::io::Error> {
        for i in 0..branch.files_changelogs.len(){
            if branch.files_changelogs[i].hash_changelog.contains(&file_to_verify){
                Ok(i)
            }
        }
        Error("File doesnt exists")
    }

    fn delete_file(file_to_remove: &str, branch:BranchChangesLog, file_index: i32) -> Result<(), std::io::Error> {
        let path_file_to_remove = format!("{}{}{}",branch.files_changelogs[index].hash_files_path, hash,format);
        fs::remove_file(path_file_to_remove)?;
        Ok(())
    }

    fn remove_file_info(file_info_to_remove: &str, branch: BranchChangesLog, file_index: i32) -> Result<(), std::io::Error> {
        branch.files_changelogs.remove(file_index);
        Ok(())
    }

    // Function to remove a file from the staging area
    pub fn remove(file_to_remove: &str,branch: &str) -> Result<(), std::io::Error> {
        let f = std::fs::File::open("./my_vcs/changelogs.yml").expect("Could not open changelog file.")?;
        let Branches: BranchChangesLog = serde_yaml::from_reader(f).expect("Could not read Branch Change Log")?;
        let index = verify_if_file_exist(f,Branches).unwrap();
        delete_file(f,Branches,index)?;
        remove_file_info(f,Branches,index)?;
        Ok(())
    }
}
