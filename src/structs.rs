use std::{fs, io};

use serde_yaml;
use serde::{Serialize, Deserialize};


/// Contains the structures used for serialization and deserialization.
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
        pub version: u32,
        pub parent_version: u32,
    }

    #[derive(Debug, Clone, Serialize, Deserialize)]
    pub struct Commit {
        pub files_changelogs: Vec<FileChangeLog>,
        pub commit_hash: String,
        pub parent_commits: Vec<String>,
    }

    #[derive(Debug, Serialize, Deserialize)]
    pub struct Branch {
        pub branch_name: String,
        pub head_commit_hash: String,
        pub commits: Vec<Commit>,
    }

    #[derive(Serialize, Deserialize)]
    pub struct Repository {
        pub current_branch: String,
        pub branches: Vec<Branch>,
    }
}

/// Provides utility functions for reading and writing YAML serialized structures.
pub struct StructWriter;

impl StructWriter {
    /// Writes blank structures to files in YAML format.
    pub fn write_blank_structs_to_files(&self) -> Result<(), Box<dyn std::error::Error>> {
        use structs_mod::{Log, Init, Repository};

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

        let my_repo = Repository {
            current_branch: String::from("main"),
            branches: Vec::new(),
        };

        // Serialize the structs into YAML format
        let log_yaml = serde_yaml::to_string(&log)?;
        let init_yaml = serde_yaml::to_string(&init)?;
        let my_repo_yaml = serde_yaml::to_string(&my_repo)?;

        // Write the YAML strings to files
        fs::write(format!("{}{}", path, "log.yml"), &log_yaml)?;
        fs::write(format!("{}{}", path, "init.yml"), &init_yaml)?;
        fs::write(format!("{}{}", path, "my_repo.yml"), &my_repo_yaml)?;

        Ok(())
    }

    /// Reads a structure from a YAML file.
    pub fn read_struct_from_file<T>(file_path: &str) -> Result<T, Box<dyn std::error::Error>>
    where
        T: for<'de> Deserialize<'de>,
    {
        let file_contents = fs::read_to_string(file_path)?;
        let struct_data: T = serde_yaml::from_str(&file_contents)?;

        Ok(struct_data)
    }

    /// Updates a YAML file with the serialized structure.
    pub fn update_struct_file<T>(file_path: &str, struct_data: &T) -> Result<(), Box<dyn std::error::Error>>
    where
        T: Serialize,
    {
        let yaml_data = serde_yaml::to_string(struct_data)?;
        fs::write(file_path, yaml_data)?;
        Ok(())
    }
}
