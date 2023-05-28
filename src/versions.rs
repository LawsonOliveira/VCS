pub mod version_fn{
    const PATH: &str = "myvcs/";

    use crate::structs::structs_mod::{BranchChangesLog,CommitFiles};
    use diffy::{apply, Patch};
    use std::any::Any;
    use std::fs::File;
    use std::io::{self, Write};
    use std::fs;

    use crate::structs::StructWriter;

    //verify if version desired exists
    fn verify_if_version_exists(version_to_verify:&str,branch :&BranchChangesLog) -> Result< usize, std::io::Error> {
        for i in 0..branch.commits_files.len(){
            if branch.commits_files[i].commit_hash == version_to_verify{
                return Ok(i);
            }
        }
        Err(std::io::Error::new(std::io::ErrorKind::NotFound, "Version doesn't exist"))
    }

    fn overwrite_file_with_string(file_path: &str, content: &str) -> io::Result<()> {
        let mut file = File::create(file_path)?;
        file.write_all(content.as_bytes())?;
        Ok(())
    }
    
    pub fn change_version(version_to_verify:&str) -> Result< (), std::io::Error>{

        let branch :BranchChangesLog = StructWriter::read_struct_from_file(PATH).unwrap();

        //split the vector [version,last_hash] rest
        let index = verify_if_version_exists(version_to_verify, &branch).unwrap();
        let (left, right) = branch.commits_files.split_at(index + 1);
        
        for commit_file in left{

            let mut concatenated_file = "".to_string();
            let mut path_current_file = "".to_string();

            for file_change_log in commit_file.files_changelogs{
                
                let diff_path = format!("{}{}",file_change_log.hash_files_path,file_change_log.hash_changelog);
                let diff_content = fs::read_to_string(&diff_path).expect("could not read hash file");
                let patch = Patch::from_str(&diff_content).unwrap();

                concatenated_file = (apply(&concatenated_file.to_owned(), &patch).unwrap());
                path_current_file = format!("{}{}",file_change_log.original_file_path.clone(),file_change_log.original_file.clone());
            }
            
            //save the version desired in the working file
            overwrite_file_with_string(&path_current_file,&concatenated_file)?; 
        }
        Ok(())       
    } 
    //change current file with the version desired
}