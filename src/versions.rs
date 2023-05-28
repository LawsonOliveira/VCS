pub mod version_fn{
    use crate::structs::structs_mod::{BranchChangesLog,FileChangeLog};
    use diffy::{apply, Patch};
    use std::any::Any;
    use std::fs::File;
    use std::io::{self, Write};

    //verify if version desired exists
    fn verify_if_version_exists(version_to_verify:&str,branch :&BranchChangesLog) -> Result< usize, std::io::Error> {
        for i in 0..branch.files_changelogs.len(){
            if branch.files_changelogs[i].hash_changelog.contains(&version_to_verify.to_owned()){
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
    
    pub fn change_version(version_to_verify:&str,branch :&BranchChangesLog) -> Result< (), std::io::Error>{
        let index = verify_if_version_exists(version_to_verify,branch).unwrap();
        let files_changelogs : &FileChangeLog = &branch.files_changelogs[index];

        //get the position of the version
        let Some(position) = files_changelogs.hash_changelog.iter().position(|x| x == version_to_verify) else{
            panic!("Can't find version: '{version_to_verify}'");
        };

        //split the vector [version,last_hash] rest
        let (left, right) = files_changelogs.hash_changelog.split_at(position);
        
        let path_original = format!("{},{}",files_changelogs.original_file_path,files_changelogs.original_file);
        let path_diff = &files_changelogs.hash_files_path;

        //apply the first patch to a file
        let original_file = std::fs::File::open(path_original).expect("Could not open file.");
        let text_original: String = serde_yaml::from_reader(original_file).expect("Could not read values.");

        let path_file = std::fs::File::open(format!("{}{}",path_diff,left[0])).expect("Could not open file.");
        let text_diff: String = serde_yaml::from_reader(path_file).expect("Could not read values.");
        
        let text_diff_owned = text_diff.to_owned();

        let patch = Patch::from_str(&text_diff_owned).unwrap();
        
        let mut concatenated_patch: String = "".to_string();
        concatenated_patch = (apply(&text_original.to_owned(), &patch).unwrap());

        //patch all others hash to the file
        for i in 1..left.len(){
            let hash = &left[i];
            let f = std::fs::File::open(format!("{}{}",path_diff,hash)).expect("Could not open file.");
			let text_diff: String = serde_yaml::from_reader(f).expect("Could not read values.");

            let text_diff_owned = text_diff.to_owned();
            let patch = Patch::from_str(&text_diff_owned).unwrap();
            concatenated_patch = (apply(&concatenated_patch.to_owned(), &patch).unwrap());
        }
        
        //save the version desired in the working file
        let path_file_to_save : String = files_changelogs.last_file.clone();
        overwrite_file_with_string(&path_file_to_save,&concatenated_patch)?; 
        Ok(())       
    } 
    //change current file with the version desired






}