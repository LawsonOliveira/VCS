pub mod structs_mod
{
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

#[derive(Serialize, Deserialize)]
pub struct Diffs {
    pub Version_Diff: Vec<String>,
    pub Path_Diff: Vec<String>,
    pub Original_File_Path: Vec<String>,
}
// You can define other structs, enums, functions, etc. in this module
}