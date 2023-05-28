use std::fs;
use std::io;
use serde_yaml;
use serde::{Serialize, Deserialize};


pub mod structs_mod {
    use serde::{Serialize, Deserialize};

    #[derive(Serialize, Deserialize)]
    pub struct Log {
        pub action: Vec<String>,
        pub created_date: Vec<String>,
        pub created_time: Vec<String>,
    }

    #[derive(Serialize, Deserialize)]
    pub struct Init {
        pub created_date: String,
        pub created_time: String,
        pub current_path: String,
    }

    #[derive(Clone, Serialize, Deserialize)]
    pub struct FileChangeLog {
        pub original_file_path: String,
        pub original_file: String,
        pub hash_changelog: String,
        pub last_file: String,
        pub hash_files_path: String,
    }


    #[derive(Clone, Serialize, Deserialize)]
    pub struct CommitFiles {
        pub files_changelogs: Vec<FileChangeLog>,
        pub commit_hash: String,
    }

    
    #[derive(Serialize, Deserialize)]
    pub struct BranchChangesLog {
        pub branch_name: String,
        pub commits_files: Vec<CommitFiles>,
    }

}

pub struct StructWriter;
impl StructWriter {
    /// Writes blank structs to YAML files.
    ///
    /// # Errors
    ///
    /// Returns an error if there is an issue writing the files.
    pub fn write_blank_structs_to_files(&self) -> Result<(), Box<dyn std::error::Error>> {
        use structs_mod::{Log, Init, FileChangeLog, CommitFiles, BranchChangesLog};
        let path = "./my_vcs/";
        let log = Log {
            action: Vec::new(),
            created_date: Vec::new(),
            created_time: Vec::new(),
        };
        let init = Init {
            created_date: String::new(),
            created_time: String::new(),
            current_path: String::new(),
        };
        let file_change_log = FileChangeLog {
            original_file_path: String::new(),
            original_file: String::new(),
            hash_changelog: String::new(),
            last_file: String::new(),
            hash_files_path: String::new(),
        };

        let commit_files = CommitFiles {
            files_changelogs: vec![file_change_log.clone()],
            commit_hash: String::new(),
        };

        let branch_changes_log = BranchChangesLog {
            branch_name: "./main/".to_string(),
            commits_files: vec![commit_files.clone()],
        };
    
        // Serialize the structs into YAML format
        let log_yaml = serde_yaml::to_string(&log)?;
        let init_yaml = serde_yaml::to_string(&init)?;
        let file_change_log_yaml = serde_yaml::to_string(&file_change_log)?;
        let branch_changes_log_yaml = serde_yaml::to_string(&branch_changes_log)?;
    
        // Write the YAML strings to files
        fs::write(format!("{}{}", path, "log.yml"), &log_yaml)?;
        fs::write(format!("{}{}", path, "init.yml"), &init_yaml)?;
        //fs::write(format!("{}{}", path, "file_change_log.yml"), &file_change_log_yaml)?;
        fs::write(format!("{}{}", path, "branch_changes_log.yml"), &branch_changes_log_yaml)?;

        Ok(())
    }

    /// Reads a struct from a YAML file.
    ///
    /// # Arguments
    ///
    /// * `file_path` - The path of the file to read.
    ///
    /// # Errors
    ///
    /// Returns an error if there is an issue reading the file or deserializing the data.
    pub fn read_struct_from_file<T>(file_path: &str) -> Result<T, Box<dyn std::error::Error>>
    where
        T: for<'de> Deserialize<'de>,
    {   
        let file_contents = fs::read_to_string(file_path)?;
        let struct_data: T = serde_yaml::from_str(&file_contents)?;

        Ok(struct_data)
    }

    /// Updates a struct file with new data.
    ///
    /// # Arguments
    ///
    /// * `file_path` - The path of the file to update.
    /// * `struct_data` - The data to update the file with.
    ///
    /// # Errors
    ///
    /// Returns an error if there is an issue writing the file or serializing the data.
    pub fn update_struct_file<T>(file_path: &str, struct_data: &T) -> Result<(), Box<dyn std::error::Error>>
    where
        T: Serialize,
    {
        let yaml_data = serde_yaml::to_string(struct_data)?;
        fs::write(file_path, yaml_data)?;
        Ok(())
    }
}
