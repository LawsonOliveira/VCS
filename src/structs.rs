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

    #[derive(Debug, Clone, Serialize, Deserialize)]
    pub struct FileChangeLog {
        pub original_file_path: String,
        pub original_file: String,
        pub last_file: String,
        pub last_file_path: String,
        pub hash_changelog: String,
        pub hash_files_path: String,
        pub version: u64,  // Added: Version number for the file change
        pub parent_versions: Vec<u64>,  // Added: List of parent version numbers
    }

    #[derive(Debug, Clone, Serialize, Deserialize)]
    pub struct Commit {
        pub files_changelogs: Vec<FileChangeLog>,
        pub commit_hash: String,
        pub parent_commits: Vec<String>,  // Added: List of parent commit hashes
    }

    #[derive(Debug, Clone, Serialize, Deserialize)]
    pub struct Branch {
        pub branch_name: String,
        pub head_commit_hash: String,  // Added: Hash of the latest commit in the branch
        pub commits: Vec<Commit>,  // Added: List of commits in the branch
    }

    #[derive(Serialize, Deserialize)]
    pub struct Repository {
        pub current_branch: String,
        pub branches: Vec<Branch>,
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
        use structs_mod::{Log, Init, Branch, Repository};
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
        let main_branch = Branch {
            branch_name: String::from("main"),
            head_commit_hash: String::new(),  // Added: Hash of the latest commit in the branch
            commits: Vec::new()  // Added: List of commits in the branch
        };
        
        let my_repo = Repository {
            current_branch: String::from("main"),
            branches: vec![main_branch.clone()],
        };
    
        // Serialize the structs into YAML format
        let log_yaml = serde_yaml::to_string(&log)?;
        let init_yaml = serde_yaml::to_string(&init)?;
        let my_repo = serde_yaml::to_string(&my_repo)?;
    
        // Write the YAML strings to files
        fs::write(format!("{}{}", path, "log.yml"), &log_yaml)?;
        fs::write(format!("{}{}", path, "init.yml"), &init_yaml)?;
        fs::write(format!("{}{}", path, "my_repo.yml"), &my_repo)?;

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
