pub mod version_fn{
    const PATH: &str = "myvcs/";

    use crate::structs::structs_mod::{BranchChangesLog,CommitFiles};
    use diffy::{apply, Patch};
    use std::any::Any;
    use std::fs::File;
    use std::io::{self, Write};
    use std::fs;
    use std::collections::HashMap;

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
        
        let mut file_map: HashMap<String, (String, String)> = HashMap::new();


        for commit_file in left{
            for file_change_log in commit_file.files_changelogs{

                let diff_path = format!("{}{}",file_change_log.hash_files_path,file_change_log.hash_changelog);
                let diff_content = fs::read_to_string(&diff_path).expect("could not read hash file");
                let patch = Patch::from_str(&diff_content).unwrap();
                
                let path_current_file = format!("{}{}",file_change_log.original_file_path,file_change_log.original_file);
                
                if let Some(string_value) = file_map.get_mut(&file_change_log.original_file) {
                    // Key is present, insert the difference between the patch inside and the current patch
                    string_value.0 = (apply(&string_value.0.to_owned(), &patch).unwrap());
                    string_value.1 = path_current_file;
                } else {
                    // Key is not present, insert the difference between a empty string and the current patch
                    let patch = apply("", &patch).unwrap();
                    file_map.insert(file_change_log.original_file.clone(), (patch,path_current_file));
                    
                }
            }
        }
        for (original_file, (patch, hash_files_path)) in file_map.iter() {
            overwrite_file_with_string(&hash_files_path,&patch)?; 
        }
        Ok(())       
    }
    //change current file with the version desired
}